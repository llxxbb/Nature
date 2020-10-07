# Nature 数据定义

这里列出了 Nature 接口和 `Executor` 接口所用到的数据定义

## ConverterParameter

用于 `Executor` 的 `convert` 接口的入参

**rust 形式** ：

```rust
pub struct ConverterParameter {
    pub from: Instance,					// 上游 Instance，请参考下面的 Instance
    pub last_state: Option<Instance>,	// 当前 Instance 的上一次状态，请参考下面的 Instance
    pub task_id: u64,					// 当前任务id，延时处理时回调 Nature 的凭据。
    pub master: Option<Instance>,		// 上游的mater Instance
    pub cfg: String,					// 为 Executor.settings 的值,用于指导当前 Executor 如何工作，是relation数据表 settings字段中的一部分。
}
```

**json 形式**：

```json
{
    "from": {},					// 上游 Instance, 请参考下面的 Instance
    "last_state": {},			// 可为 null，当前数据实例的上一次状态, 请参考下面的 Instance
    "task_id": 123,				// 当前任务id，延时处理时回调 Nature 的凭据。
    "master": {},				// 上游的 mater Instance 请参考下面的 nstance json
    "cfg": ""					// 为 Executor.settings 的值,用于指导当前 Executor 如何工作，是relation 数据表 settings字段中的一部分。
}
```

## Instance 

[Meta](meta.md) 的运行时

**rust 形式** ：

```rust
pub struct Instance {
    pub id: u64,			// 唯一标识
    pub data: BizObject,	// 业务对象的具体内容
    pub create_time: i64,	// 创建时间
}
```

**json 形式**：

```json
{
    "id":12343,				// 唯一标识
	"data": {},				// 业务对象的具体内容,请参考 BizObject json
	"create_time": 133231	// 创建时间, long 类型
}
```

## BizObject

`Instance` 的具体内容

**rust 形式** ：

```rust
pub struct BizObject {
    pub meta: String,		// 所属 Meta 的 meta-string 形式
    pub content: String,	// 业务具体内容
    pub context: HashMap<String, String>,		// 服务于流程控制的其它业务信息。
    pub sys_context: HashMap<String, String>, 	// 同 context 只是这个上下文里的内容是由 Nature 进行规范的。
    pub states: HashSet<String>,	// 业务的状态信息。其值必须在 Meta 中进行定义才能使用
    pub state_version: i32,			// 状态信息的版本
    pub from: Option<FromInstance>,	// 上游 Instance，请见下方 FromInstance
    pub para: String,				// 另一唯一标记当前 Instance 的方法，是关联外部数据的有力工具
}
```

**json 形式**：

```json
{
    "meta": "B:sale/order:1",		// 所属 Meta 的 meta-string 形式
    "content": "item:[...],price:123,...",		// 业务具体内容
    "context": {"k1":"v1"...},	// 缺省 null， Key-Value结构，Value为String类型，服务于流程控制的其它业务信息。
    "sys_context": {"k1":"v1"...},	// 同 context 只是这个上下文里的内容是由 Nature 进行规范的。
    "states": ["s1"...],		// 缺省 null。业务的状态信息。其值必须在 Meta 中进行定义才能使用
    "state_version": 0,			// 状态信息的版本
    "from": {},					// 缺省 null，上游 Instance，请见下方 FromInstance
    "para": "",					// 缺省 null，另一唯一标记当前 Instance 的方法，是关联外部数据的有力工具
}
```

## FromInstance

是一种简化了的 `Instance`, 用于表示上游 `Instance`

**rust 形式** ：

```rust
pub struct FromInstance {
    pub id: u64,					// 请参考上方Instance.id
    pub meta: String,				// 请参考上方 BizObject.meta
    pub para: String,				// 请参考上方 BizObject.para
    pub state_version: 0,			// 请参考上方 BizObject.state_version
}
```

**json 形式**：

```rust
{
    "id": 123,					// 请参考 Instance.id
    "meta": "B:shop/cart:1",	// 请参考上方 BizObject.meta
    "para": null,				// 请参考上方 BizObject.para
    "state_version": 0,			// 请参考上方 BizObject.state_version
}
```

## ConverterReturned

Executor 的处理结果

**rust 形式** ：

```rust
pub enum ConverterReturned {
    LogicalError(String),				// 逻辑错误，Nature 不会重试
    EnvError(String),					// 环境错误，Nature 会按照重试策略进行重试
    None,								// 没有 Instance 生成
    Delay(u32),							// 用于延时（秒）处理，具体用法请看Demo
    Instances(Vec<Instance>),			// 产出的 Instance
    SelfRoute(Vec<SelfRouteInstance>),	// 产出含有动态路由的 Instance
}
```

**json 形式**：

返回生成的 `Instance` ，其中 `ins` 属性是 `Instance` 数组，结构说明请看上方的 `Instance`

```json
{"type":"Instances","ins":[]}
```

如果不需要返回任何 `Instance` ，则请返回下面的 json

```json
{"type":"None"}
```

如果遇到逻辑错误请返回下面的 json，Nature 将放弃该任务！

```json
{"type":"LogicalError","msg":"some error"}
```

如果遇到环境错误请返回下面的 json，Nature 会按照重试策略进行重试。

```json
{"type":"EnvError","msg":"some error"}
```

如果需要进行异步处理，则可以立即返回下面的信息给 Nature，Nature 将在您给定的时间内不会发起重试，请您在您自己指定的时间内完成转换任务并回调[Nature 接口](nature-interface.md)中的`/callback`接口。这里`num` 的单位是秒。

```json
{"type":"Delay","num":10}
```

返回自带路由的数据请以下面的形式进行返回，其中 `ins` 为 SelfRouteInstance 数组，其结构请看下方的 SelfRouteInstance。

```json
{"type":"SelfRoute","ins":[]}
```

## SelfRouteInstance

自带路由信息的 `Instance`

**rust 形式** ：

```rust
pub struct SelfRouteInstance {
    pub instance: Instance,					// 生成的 Instance
    pub converter: Vec<DynamicConverter>,	// 见下面的 DynamicConverter
}
```

**json 形式**：

```rust
{
    "instance": {},			// 生成的 Instance, 请参考上方的 instance
    "converter": []			// 为 DynamicConverter 数组，请看下方的 DynamicConverter
}
```

## DynamicConverter

动态转换器，用于 `SelfRouteInstance`

**rust 形式** ：

```rust
pub struct DynamicConverter {
    pub to: Option<String>,		// 目标 Meta，只支持 MetaType::Dynamic 和 MetaType::Null
    pub fun: Executor,			// 用于执行转换任务的 Executor，见下面的 Executor
    pub use_upstream_id: bool,	// 缺省 false, 是否使用上游 Instance.id 作为生成 Instance 的 id
    pub delay: i32,				// 缺省 0，Executor 需要延时执行的时间，单位：秒
}
```

**json 形式**：

```json
{
    "to": "D:targetMeta:1",	// 目标 Meta，只支持 MetaType::Dynamic 和 MetaType::Null
    "fun": {},				// 用于执行转换任务的 Executor，见下面的 Executor
    "use_upstream_id": true,// 缺省 false, 是否使用上游 Instance.id 作为生成 Instance 的 id
    "delay": 10				// 缺省 0，Executor 需要延时执行的时间，单位：秒
}
```

## Executor

Nature `Executor` 的定义

**rust 形式** ：

```rust
pub struct Executor {
    pub protocol: Protocol,	// 见下面的 Protocol
    pub url: String,		// Executor 坐标
    pub settings: String,	// Executor 自身的配置。
}
pub enum Protocol {
    LocalRust,				// rust lib 包
    Http,
    Https,
    Auto,	 				// Nature 会自动生成 Executor, 您不能直接使用此协议
    BuiltIn,				// 使用 Nature 内置的 Executor
}
```

**json 形式**：

```json
{
    "protocol": "http",		// 可选的协议：http|https|localRust|builtin
    "url": String,			// Executor 坐标
    "settings": String		// Executor 自身的配置。
}
```

## DelayedInstances

`Executor` 回调 Nature 的入参

**rust 形式** ：

```rust
pub struct DelayedInstances {
    pub task_id: u64,		// Nature 分配给 Executor 的任务ID
    pub result: ConverterReturned,	// 见上面的 ConverterReturned
}
```

**json 形式**：

```json
{
    "task_id": 123,		// Nature 分配给 Executor 的任务ID
    "result": {}		// 见上方的 ConverterReturned
}
```

## KeyCondition

用于查询 Nature 中的 Instance 

**rust 形式** ：

```rust
pub struct KeyCondition {
    pub id: u64,			// instance id 
    pub meta: String,		// meta-string
    pub key_gt: String,		// 用于批查询，结果的 key 要大于此值
    pub key_ge: String,		// 用于批查询，结果的 key 要大于等于此值
    pub key_lt: String,		// 用于批查询，结果的 key 要小于此值
    pub key_le: String,		// 用于批查询，结果的 key 要小于等于此值
    pub para: String,		// instance.para
    pub state_version: i32,	// instance.state_version
    pub time_ge: Option<i64>,	// 创建时间有要大于等于此值
    pub time_lt: Option<i64>,	// 创建时间有要小于此值
    pub limit: i32,				// 返回结果的数量
}
```

其中 key 的构成为 “meta-string|id|para”

**json 形式**：

```json
{
    "id": 123,			// instance id 
    "meta": "B:sale/Order:1",	// meta-string
    "key_gt": "",		// 用于批查询，结果的 key 要大于此值
    "key_ge": "",		// 用于批查询，结果的 key 要大于等于此值
    "key_lt": "",		// 用于批查询，结果的 key 要小于此值
    "key_le": "",		// 用于批查询，结果的 key 要小于等于此值
    "para": "",			// instance.para
    "state_version": 0,	// instance.state_version
    "time_ge": 123		// 创建时间有要大于等于此值
    "time_lt": 456,		// 创建时间有要小于此值
    "limit": 1,			// 返回结果的数量
}
```

