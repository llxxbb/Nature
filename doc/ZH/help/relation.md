# Relation

用于实现目标之间的转换，其定义存储到数据表 relation 中

## 存储 `Relation`

示例如下：

```sql
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/order:1', 'B:sale/orderState:1', '{"target_states":{"add":["new"]}}');
```

## 定义 `Relation` 的处理方式

上面的示例 SQL 中的 settings 字段用于对每个关系的处理方式进行个性化定义。settings 的值是 JSON形式的 `RelationSettings` 对象，其结构如下。

```rust
pub struct RelationSettings {
    pub selector: Option<FlowSelector>,
    pub executor: Option<Executor>,
    pub filter_before: Vec<Executor>,
    pub filter_after: Vec<Executor>,
    pub use_upstream_id: bool,
    pub target: RelationTarget,
    pub delay: i32,
    pub delay_on_para: (i32, u8),
}
```

- selector：属性用于选择符合条件的 `Instance` 进入 `Executor` 进入处理，其结构见下方 `FlowSelector`的结构说明。
- executor：属性用于定义谁来做这个转化处理，其结构见下方 `Executor`的结构说明。
- filter_before: 在executor之前执行用于对输入实例进行修正。可以是多个，按给定的顺序执行。
- filter_after: 在executor之后执行用于对结果进行修正。可以是多个，按给定的顺序执行。
- use_upstream_id：新生成的 `Instance` 的 ID 将使用上游 `Instance`的 ID。
- target：对目标实例的一些要求，下面会有具体解释。
- delay：本次任务需要延迟指定的秒数后执行。
- delay_on_para：延迟本次任务的执行，延迟的时间=上游`Instance.para`中指定的位置的时间（元组中的第二个值）+给定的延时时间（元组中的第一个值）

### 触发转换的条件： FlowSelector

```rust
pub struct FlowSelector {
    pub state_all: HashSet<String>,
    pub state_any: HashSet<String>,
    pub state_none: HashSet<String>,
    pub context_all: HashSet<String>,
    pub context_any: HashSet<String>,
    pub context_none: HashSet<String>,
    pub sys_context_all: HashSet<String>,
    pub sys_context_any: HashSet<String>,
    pub sys_context_none: HashSet<String>,
}

```


all of above are `and` relation

- state_[...]：上游 `Instance` 的状态必须满足[]中的要求。
- context_[...]：上游 `Instance` 的上下文必须满足[]中的要求。
- sys_context_[...]：上游`Instance`的系统上下文必须满足[]中的要求。

优先级

```
/// none: means can't include any one
/// all : means must include all
/// any : means must include one
```

### 定义用于转化的：Executor

```rust
pub struct Executor {
    pub protocol: Protocol,
    pub url: String,
    pub settings: String,
}
```

**protocol**： Nature 与 `Executor`间的通讯协议，目前支持下面的方式。

- Http | Https：远程调用一个`Executor`。
- LocalRust：Nature 会加载一个本地 rust 库作为`Executor`
- Auto：不能显式设置此值。服务于Nature自动创建的`Executor`
- BuiltIn：使用Nature 内置的转换器进行转换。

**url**：用于定位`Executor`的位置

**settings**:`Executor`专有的配置，由具体的`Executor`给出。

`Executor`的示例

```json
{
    "protocol":"Http",
    "url":"http://some_domain:8081/some_converter"
}
```

```json
{
    "protocol":"LocalRust",
    "url":"some_lib:some_converter"
}
```

### RelationTarget

```
pub struct RelationTarget {
    pub states: Option<TargetState>,
    pub upstream_para: Vec<u8>,
}
```

target_states：可以增加或删除转化后的 `Instance` 的状态，状态必须在 `Meta` 中定义过。

upstream_para：该属性可指导 Nature 如何生成目标实例的 `para` 属性。示例，如其值为[3,1]， 假设上游para为 “a/b/c/d”，则目标实例的 `para` 值为 “d/b”。

### 对目标状态的处理及要求：TargetState

```rust
pub struct TargetState {
    pub add: Option<Vec<String>>,		// 在上个状态基础上增加新的状态
    pub remove: Option<Vec<String>>,	// 从上个状态中删除指定的状态
    pub need_all: HashSet<String>,		// 上个目标状态必须拥有指定的状态
    pub need_any: HashSet<String>,		// 上个目标状态必须有一个或多个指定的状态
    pub need_none: HashSet<String>,		// 上个目标状态中不能含有任何一个指定的状态
}
```

# Executor

`Executor` 用于实现 `Meta` 间 `Instance` 的转换，一般需要自己实现，Nature 也有内建及自动化的 `Executor` 实现。实现方式请参考[示例及功能讲解](https://github.com/llxxbb/Nature-Demo)。

`Executor`只有一个入参和一个出参。

### 入参：ConverterParameter

```rust
pub struct ConverterParameter {
    pub from: Instance,					// 上游数据实例
    pub last_state: Option<Instance>,	// 最近一次状态目标的数据实例
    pub task_id: Vec<u8>,				// 此次任务ID，延时处理时回调Nature的凭据。
    pub master: Option<Instance>,		// 上游 mater的数据实例（ID相同）
    pub cfg: String,					// json 对象，`Executor`自有的配置。
}
```

### 出参：

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

## filter_before 接口形式

filter_before 需要使用者自行实现,下面为LocalRust的实现形式

```rust
#[no_mangle]
#[allow(improper_ctypes)]
pub extern fn your_func(para: &Instance) -> Result<Instance> {
	// TODO your logic
}
```

## filter_after 接口形式

filter_after 需要使用者自行实现,下面为LocalRust的实现形式

```rust
#[no_mangle]
#[allow(improper_ctypes)]
pub extern fn your_func(para: &Vec<Instance>) -> Result<Vec<Instance>> {
	// TODO your logic
}
```

### 动态`Executor`

动态路由不需要在运行之前预先定义，既在运行时决定自己的去处，非常的灵活，每个实例可以有自己独立的选择。不过不建议使用，一是目前此功能还不完善，二是该功能性能比静态路由要差，三、业务布局的展示会比较困难。

