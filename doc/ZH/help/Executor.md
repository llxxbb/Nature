# Executor接口形式

### converter接口形式

转换器接口形式的`Executor` 用于实现 `Meta` （业务对象）间 `Instance` （数据实例）的转换，一般需要自己实现，Nature 也有内建及自动化的转换器`Executor` 实现。内建和自动化的转换器`Executor`请参考[示例及功能讲解](https://github.com/llxxbb/Nature-Demo)。

转换器`Executor`只有一个入参和一个出参。

**入参：ConverterParameter**

```rust
pub struct ConverterParameter {
    pub from: Instance,					// 上游数据实例
    pub last_state: Option<Instance>,	// 最近一次状态目标的数据实例
    pub task_id: Vec<u8>,				// 此次任务ID，延时处理时回调Nature的凭据。
    pub master: Option<Instance>,		// 上游 mater的数据实例（ID相同）
    pub cfg: String,					// json 对象，`Executor`自有的配置。
}
```

**出参：**

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

### filter_before 接口形式

filter_before 需要使用者自行实现,下面为LocalRust的实现形式

```rust
#[no_mangle]
#[allow(improper_ctypes)]
pub extern fn your_func(para: &Instance) -> Result<Instance> {
	// TODO your logic
}
```

### filter_after 接口形式

filter_after 需要使用者自行实现,下面为LocalRust的实现形式

```rust
#[no_mangle]
#[allow(improper_ctypes)]
pub extern fn your_func(para: &Vec<Instance>) -> Result<Vec<Instance>> {
	// TODO your logic
}
```

## 动态`Executor`转换器

动态路由不需要在运行之前预先定义，既在运行时决定自己的去处，非常的灵活，每个实例可以有自己独立的选择。不过不建议使用，一是目前此功能还不完善，二是该功能性能比静态路由要差，三、业务布局的展示会比较困难。