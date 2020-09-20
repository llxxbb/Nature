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

**ConverterParameter结构定义如下** ：

```rust
pub struct ConverterParameter {
    pub from: Instance,					// 上游数据实例
    pub last_state: Option<Instance>,	// 最近一次状态目标的数据实例
    pub task_id: Vec<u8>,				// 此次任务ID，延时处理时回调Nature的凭据。
    pub master: Option<Instance>,		// 上游 mater的数据实例（ID相同）
    pub cfg: String,					// json 对象，`Executor`自有的配置。
}
```

**ConverterReturned结构定义如下**：

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

**Instance的结构定义如下**：

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

**BizObject的结构定义如下**：

```rust
pub struct BizObject {
    pub meta: String,		// 该业务对象实例所属的 meta 定义
    pub content: String,	// 业务对象的具体内容
    pub context: HashMap<String, String>,	// 存储业务对象之外的其它业务信息，这些信息可能会影响流程走向，可能会影响下游数据的处理方式。
    pub sys_context: HashMap<String, String>, // 同 context 只是这个上下文里的内容是由 Nature 进行规范的。
    pub states: HashSet<String>,	// 业务对象数据实例的业务状态描述。
    pub state_version: i32,			// 标记当前的状态变化是第几个版本
    pub from: Option<FromInstance>,	// 当前数据实例的上游数据实例
    pub para: String,				// 另一种唯一标记该数据实例的方式，是关联外部数据的有力工具
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

## 动态`Executor`转换器（实验阶段）

动态路由不需要在运行之前预先定义，既在运行时决定自己的去处，非常的灵活，每个实例可以有自己独立的选择。不过不建议使用，一是目前此功能还不完善，二是该功能性能比静态路由要差，三、业务布局的展示会比较困难。

## 示例

本文档中所涉及到所有内容都可以在 [Nature-Demo](https://github.com/llxxbb/Nature-Demo) 项目中找到对应的示例。

