# Executor接口形式

Executor 用于执行用户定制的逻辑，除了[内建 Executor](built-in.md) 和自动化的 Executor外，都需要用户按照给定的规范进行实现，Nature 目前有三种形式的 `Executor`：

- converter 形式，用于业务对象（`Meta`）间数据实例（`Instance`）的转换。
- convert_before 形式，用于业务对象转换前的预处理，如数据格式的修正，数据加载等。
- convert_after 形式，用于业务对象转换后的后置处理。

其实从功能上讲 `convert_before` 和 `convert_after` 完全可以用 `converter` 形式来替换但不建议，有下面的原因：

- `convert_before` 和 `convert_after` 一般只是技术上的处理，没有超出业务对象的范围，而 converter 则是衔接了两个业务对象。这个区别将来会用于业务对象关系的展示，既`convert_before` 和 `convert_after`是不会体现在这个界面上的。
- 从性能上讲 `converter` 与 `converter` 之间会有一个转换后的存储过程，而 `convert_before` ,`convert_after` 和 `converter` 之间是不需要存储的，效率会更高。

目前 Nature 可以通过两种方式来调用`Executor`  ： http 和 localRust。

## http|https协议实现方式

请用一种自己喜欢的语言来实现一个可处理POST请求的 Web 服务。并确保 Nature 可以按照`关系`中定义的`url`访问到这个服务地址。

### converter 接口形式

该接口接收一个 json 输入，并产生一个 json 输出。

#### 入参的json结构定义如下

```json
{
    "from": {},					// 上游数据实例, 请参考下面的 Instance json
    "last_state": {},			// 可为null，当前数据实例的上一次状态, 请参考下面的 nstance json
    "task_id": 123,				// 此次任务ID，延时处理时回调Nature的凭据。
    "master": {},				// 上游的mater数据实例（ID相同） 请参考下面的 nstance json
    "cfg": ""					// 可为 null 为 Executor.settings 的值,用于指导该接口如何工作，是relation 数据表settings字段中的一部分。
}
```

**Instance** json

```json
{
    "id":12343,				// 用于唯一标识这个业务对象的值，
	"data": {},				// 业务对象的具体内容,请参考 BizObject json
	"create_time": 133231	// 创建时间
}
```

**BizObject** json

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

**FromInstance** 为上游数据实例，其json结构定义如下：

```rust
{
    "id": 123,					// 请参考 Instance.id
    "meta": "B:shop/cart:1",	// 上游业务对象实例所属的 meta 定义
    "para": String,				// 请参考 BizObject.para
    "state_version": 0,			// 请参考 BizObject.state_version
}
```

#### 出参的json结构定义如下：

用下面的方式返回转换结果，其中 `ins` 属性是 `instance` 数组，结构说明请看上方的 instance json

```json
{"type":"Instances","ins":[]}
```

如果不需要返回数据实例，则请返回下面的 json

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

如果需要进行异步处理，则可以立即返回下面的信息给 Nature，Nature 将在您给定的时间内不会发起重试，请您在您自己指定的时间内完成转换任务并回调 [Nature 的回调接口](callback.md)。这里`num` 的单位是秒。

```json
{"type":"Delay","num":10}
```

返回自带路由的数据请以下面的形式进行返回，其中 `ins` 为 SelfRouteInstance 数组，其结构请看下方的 SelfRouteInstance json。

```json
{"type":"SelfRoute","ins":[]}
```

**SelfRouteInstance** json

```rust
{
    "instance": {},			// 请参考上方的 instance json
    "converter": []			// 为 DynamicConverter 数组，请看下方的DynamicConverter json
}
```

**DynamicConverter** 动态转换器，其 json结构如下：

```json
{
    "to": "D:targetMeta:1",	// Only `Dynamic` and `Null` metaType support
    "fun": {},				// 请看下方的 Executor json
    "use_upstream_id": true,// 缺省 false, 是否使用上游实例ID 作为自身的ID
    "delay": 10				// 缺省 0，Executor 需要延时执行的时间
}
```

**Executor** json 结构如下

```json
{
    "protocol": "http",		// 可选的协议：http|https|localRust|builtin
    "url": String,			// executor 坐标
    "settings": String		// executor 自身的配置。
}
```

### convert_before 接口形式

该接口接收一个 json 输入，并产生一个 json 输出。入参为 `instance` 对象，请见上方的 instance json。出参有两种情况，如下：

1. 正常情况下请输出下面的内容，Ok 的值为改变后的 instance 对象，请见上方的 instance json

   ```json
   {"Ok":{}}
   ```

2. 如果处理过程中遇到问题，则返回下面的内容:

   ```json
   {"Err":{"LogicalError":"err message"}}	
   ```

   或

   ```json
   {"Err":{"EnvironmentError":"err message"}}	
   ```

### convert_after 接口形式

该接口接收一个 json 输入，并产生一个 json 输出。接口实现形式如下，入参为 `instance` 对象数组，请见上方的 instance json。出参有两种情况，如下：

1. 正常情况下请输出下面的内容，Ok 的值为改变后的 instance 对象数组，请见上方的 instance json

   ```json
   {"Ok":[]}
   ```

2. 如果处理过程中遇到问题，则返回下面的内容:

   ```json
   {"Err":{"LogicalError":"err message"}}	
   ```

   或

   ```json
   {"Err":{"EnvironmentError":"err message"}}	
   ```

## localRust协议实现方式

为本地 lib 包。用 Rust 创建一个 lib 项目。cargo.toml 需要包含类似于下面的内容。

```toml
[lib]
name="your_lib"
crate-type = ["cdylib"]
```

### converter 接口形式

请按下面的示例代码进行实现

```rust
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern fn your_func(para: &ConverterParameter) -> ConverterReturned {
	// your logic
}
```

**ConverterParameter**结构定义如下 ：

```rust
pub struct ConverterParameter {
    pub from: Instance,					// 上游数据实例
    pub last_state: Option<Instance>,	// 当前数据实例的上一次状态
    pub task_id: u64,				// 此次任务ID，延时处理时回调Nature的凭据。
    pub master: Option<Instance>,		// 上游的mater数据实例（ID相同）
    pub cfg: String,					// 为 Executor.settings 的值,用于指导该接口如何工作，是relation数据表settings字段中的一部分。
}
```

**ConverterReturned**结构定义如下：

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

### convert_before 接口形式

接口实现形式如下：

```rust
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern fn your_func(para: &Instance) -> Result<Instance> {
	// TODO your logic
}
```

**Instance**的结构定义如下：

```rust
pub struct Instance {
    pub id: ID,				// 用于唯一标识这个业务对象的值，
    pub data: BizObject,	// 业务对象的具体内容
    pub create_time: i64,	// 创建时间
}
```

其中`ID`会依据 Nature 编译时选取的 feature 不同而不同。

- 如果 feature = id64 （缺省）则ID为u64
- 如果 feature = id128 则ID为u128

**BizObject**的结构定义如下：

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

**FromInstance** 为上游数据实例，其结构定义如下：

```rust
pub struct FromInstance {
    pub id: ID,						// 上游 ID 见上方的Instance 说明
    pub meta: String,				// 上游业务对象实例所属的 meta 定义
    pub para: String,				// 请参考 BizObject.para
    pub state_version: 0,			// 请参考 BizObject.state_version
}
```

### convert_after 接口形式

接口实现形式如下，Instance 定义同上。

```rust
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern fn your_func(para: &Vec<Instance>) -> Result<Vec<Instance>> {
	// TODO your logic
}
```

## 示例

本文档中所涉及到所有内容都可以在 [Nature-Demo](https://github.com/llxxbb/Nature-Demo) 项目中找到对应的示例。

