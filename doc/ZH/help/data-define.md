# Nature 数据定义

这里列出了 Nature 接口和 Executor 接口所用到的数据

## ConverterParameter

用于 Executor 的 convert 接口的入参

**rust 形式如下** ：

```rust
pub struct ConverterParameter {
    pub from: Instance,					// 上游数据实例
    pub last_state: Option<Instance>,	// 当前数据实例的上一次状态
    pub task_id: u64,				// 此次任务ID，延时处理时回调Nature的凭据。
    pub master: Option<Instance>,		// 上游的mater数据实例（ID相同）
    pub cfg: String,					// 为 Executor.settings 的值,用于指导该接口如何工作，是relation数据表settings字段中的一部分。
}
```

**json 形式如下**：

```json
{
    "from": {},					// 上游数据实例, 请参考下面的 Instance json
    "last_state": {},			// 可为null，当前数据实例的上一次状态, 请参考下面的 nstance json
    "task_id": 123,				// 此次任务ID，延时处理时回调Nature的凭据。
    "master": {},				// 上游的mater数据实例（ID相同） 请参考下面的 nstance json
    "cfg": ""					// 可为 null 为 Executor.settings 的值,用于指导该接口如何工作，是relation 数据表settings字段中的一部分。
}
```

## Instance 

表示业务对象的实例

**rust 形式如下** ：

```rust
pub struct Instance {
    pub id: u64,				// 用于唯一标识这个业务对象的值，
    pub data: BizObject,	// 业务对象的具体内容
    pub create_time: i64,	// 创建时间
}
```

**json 形式如下**：

```json
{
    "id":12343,				// 用于唯一标识这个业务对象的值，
	"data": {},				// 业务对象的具体内容,请参考 BizObject json
	"create_time": 133231	// 创建时间
}
```

## BizObject

表示业务对象的实例的具体内容

**rust 形式如下** ：

```rust
pub struct BizObject {
    pub meta: String,		// 该业务对象实例所属的 meta 定义
    pub content: String,	// 业务对象的具体内容
    pub context: HashMap<String, String>,	// 存储业务对象之外的其它业务信息，这些信息可能会影响流程走向，可能会影响下游数据的处理方式。
    pub sys_context: HashMap<String, String>, // 同 context 只是这个上下文里的内容是由 Nature 进行规范的。
    pub states: HashSet<String>,	// 业务对象数据实例的业务状态描述。其值必须在 Meta 中进行定义才能使用
    pub state_version: i32,			// 标记当前的状态变化是第几个版本
    pub from: Option<FromInstance>,	// 当前数据实例的上游数据实例，结构请见下方
    pub para: String,				// 另一种唯一标记该数据实例的方式，是关联外部数据的有力工具
}
```

**json 形式如下**：

```json
{
    "meta": "B:sale/order:1",		// 该业务对象实例所属的 meta 定义
    "content": "item:[...],price:123,...",		// 业务对象的具体内容
    "context": {"k1":"v1"...},	// 可为null， Key-Value结构，Value为String类型，存储业务对象之外的其它业务信息，这些信息可能会影响流程走向，可能会影响下游数据的处理方式。
    "sys_context": {"k1":"v1"...},	// 可为null， Key-Value结构，Value为String类型，同 context 只是这个上下文里的内容是由 Nature 进行规范的。
    "states": ["s1"...],		// 业务对象数据实例的业务状态描述。其值必须在 Meta 中进行定义才能使用
    "state_version": 0,			// 标记当前的状态变化是第几个版本
    "from": Option<FromInstance>,	// 可为 null 当前数据实例的上游数据实例
    "para": String,				// 另一种唯一标记该数据实例的方式，是关联外部数据的有力工具
}
```

## FromInstance

**FromInstance** 为上游数据实例。

**rust 形式如下** ：

```rust
pub struct FromInstance {
    pub id: ID,						// 上游 ID 见上方的Instance 说明
    pub meta: String,				// 上游业务对象实例所属的 meta 定义
    pub para: String,				// 请参考 BizObject.para
    pub state_version: 0,			// 请参考 BizObject.state_version
}
```

**json 形式如下**：

```rust
{
    "id": 123,					// 请参考 Instance.id
    "meta": "B:shop/cart:1",	// 上游业务对象实例所属的 meta 定义
    "para": String,				// 请参考 BizObject.para
    "state_version": 0,			// 请参考 BizObject.state_version
}
```

## ConverterReturned

Executor 的处理结果

**rust 形式如下** ：

```rust
pub enum ConverterReturned {
    LogicalError(String),				// 逻辑错误，Nature 不会重试
    EnvError(String),					// 当前条件不满足，Nature 会在将来的某个时刻重试
    None,								// 没有数据返回
    Delay(u32),							// 用于延时处理，具体用法请看Demo
    Instances(Vec<Instance>),			// 产出的目标数据实例
    SelfRoute(Vec<SelfRouteInstance>),	// 定义动态路由
}
```

**json 形式如下**：

返回生成的 instance ，其中 `ins` 属性是 `instance` 数组，结构说明请看上方的 instance json

```json
{"type":"Instances","ins":[]}
```

如果不需要返回任何 instance 数据实例，则请返回下面的 json

```json
{"type":"None"}
```

如果遇到逻辑错误请返回下面的 json，Nature 将放弃该任务！

```json
{"type":"LogicalError","msg":"some error"}
```

如果遇到环境错误请返回下面的 json，Nature 会在将来的某个时间进行重试。

```json
{"type":"EnvError","msg":"some error"}
```

如果需要进行异步处理，则可以立即返回下面的信息给 Nature，Nature 将在您给定的时间内不会发起重试，请您在您自己指定的时间内完成转换任务并回调 [Nature 接口](nature-interface.md)中的回调接口。这里`num` 的单位是秒。

```json
{"type":"Delay","num":10}
```

返回自带路由的数据请以下面的形式进行返回，其中 `ins` 为 SelfRouteInstance 数组，其结构请看下方的 SelfRouteInstance json。

```json
{"type":"SelfRoute","ins":[]}
```

## SelfRouteInstance

自带路由信息的 Instance

**rust 形式如下** ：

```rust
pub struct SelfRouteInstance {
    pub instance: Instance,
    pub converter: Vec<DynamicConverter>,
}
```

**json 形式如下**：

```rust
{
    "instance": {},			// 请参考上方的 instance json
    "converter": []			// 为 DynamicConverter 数组，请看下方的DynamicConverter json
}
```

## DynamicConverter

动态转换器，用于 `SelfRouteInstance`

**rust 形式如下** ：

```rust
pub struct DynamicConverter {
    pub to: Option<String>,		// 目标元数据，只支持 MetaType::Dynamic 和 MetaType::Null
    pub fun: Executor,			// 用于执行转换任务的执行器
    pub use_upstream_id: bool,	// 缺省 false, 是否使用上游实例ID 作为自身的ID
    pub delay: i32,				// 缺省 0，Executor 需要延时执行的时间
}
```

**json 形式如下**：

```json
{
    "to": "D:targetMeta:1",	// 目标元数据，只支持 MetaType::Dynamic 和 MetaType::Null
    "fun": {},				// 用于执行转换任务的执行器，请看下方的 Executor json
    "use_upstream_id": true,// 缺省 false, 是否使用上游实例ID 作为自身的ID
    "delay": 10				// 缺省 0，Executor 需要延时执行的时间
}
```

## Executor

Nature 执行器的定义

**rust 形式如下** ：

```rust
pub struct Executor {
    pub protocol: Protocol,	// 可选的协议
    pub url: String,		// executor 坐标
    pub settings: String,	// executor 自身的配置。
}
pub enum Protocol {
    LocalRust,				// rust lib 包
    Http,
    Https,
    Auto,	 				// it can't be used by user.
    BuiltIn,
}
```

**json 形式如下**：

```json
{
    "protocol": "http",		// 可选的协议：http|https|localRust|builtin
    "url": String,			// executor 坐标
    "settings": String		// executor 自身的配置。
}
```

## DelayedInstances

Executor 回调 Nature 的入参

**rust 形式如下** ：

```rust
pub struct DelayedInstances {
    pub task_id: u64,		// Nature 分配给 Executor 的任务ID
    pub result: ConverterReturned,
}
```

**json 形式如下**：

```json
{
    "task_id": 123,		// Nature 分配给 Executor 的任务ID
    "result": {}		// 参考上方的 ConverterReturned
}
```

## KeyCondition

用于查询 Nature 中的 Instance 

**rust 形式如下** ：

```rust
pub struct KeyCondition {
    pub id: String,			// 16 进制的 instance id 
    pub meta: String,		// instance 对应的元数据
    pub key_gt: String,		// 用于批查询，结果的 key 要大于此值
    pub key_ge: String,		// 用于批查询，结果的 key 要大于等于此值
    pub key_lt: String,		// 用于批查询，结果的 key 要小于此值
    pub key_le: String,		// 用于批查询，结果的 key 要小于等于此值
    pub para: String,		// instance.para
    pub state_version: i32,	// instance.state_version
    pub time_ge: Option<i64>,	// 创建时间有要大于此值
    pub time_lt: Option<i64>,	// 创建时间有要小于此值
    pub limit: i32,				// 返回结果的数量
}
```

其中 instance 的 key 为 meta|id|para 构成的字符串

**json 形式如下**：

```json
{
    "id": "1a2b",			// 16 进制的 instance id 
    "meta": "B:sale/Order:1",	// instance 对应的元数据
    "key_gt": "",		// 用于批查询，结果的 key 要大于此值
    "key_ge": "",		// 用于批查询，结果的 key 要大于等于此值
    "key_lt": "",		// 用于批查询，结果的 key 要小于此值
    "key_le": "",		// 用于批查询，结果的 key 要小于等于此值
    "para": "",			// instance.para
    "state_version": 0,	// instance.state_version
    "time_ge": 123		// 创建时间有要大于此值
    "time_lt": 456,		// 创建时间有要小于此值
    "limit": 1,			 返回结果的数量
}
```

