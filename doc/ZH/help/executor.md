# Executor

除了[内建 Executor](built-in.md) 和自动化的 `Executor` 外，您需要按照 Nature 给定的接口协议进行 `Executor` 编码实现并部署。`Executor` 须定义在 [Relation](relation.md) 里并由 Nature 负责调用。Nature 目前有三种形式的 `Executor`：

- converter: 用于 [Meta](meta.md) 间 `Instance` 的转换。
- convert_before: 用于 `Instance` 转换前的预处理，如数据格式的修正，数据加载等。
- convert_after: 用于转换后 `Instance` 的后置处理。

这三种形式的 `Executor` 都可以在 [Nature-Demo](../../../nature-demo/README.md) 项目中找到对应的示例。

其实从功能上讲 `convert_before` 和 `convert_after` 完全可以用 `converter` 形式来替换，但 Nature 不建议这样做，有下面的原因：

- `convert_before` 和 `convert_after` 一般只是技术上的处理，没有超出业务对象的范围，而 `converter` 则是衔接了两个业务对象，既超出了业务对象的范围。在管理界面上来讲，`converter` 将来会用于业务对象关系的展示，但`convert_before` 和 `convert_after`是不会被直接展示的。
- 从性能上讲 `converter` 会有一个写库的过程，而 `convert_before` , `convert_after`是不需要存储的，效率会更高，这在海量数据处理时影响较大。

## 协议

目前 Nature 可以通过两种方式来调用`Executor`  ： http 和 localRust。后续可能会补充其它协议。

### http|https协议实现方式

请用一种自己喜欢的语言来实现一个可处理POST请求的 Web 服务。并确保 Nature 可以按照 [Relation](relation.md) 中 `executor` 属性定义的`url`访问到这个服务地址。

### localRust协议实现方式

为本地 lib 包，生成的包需要与 nature 可执行文件置于同一目录下 。下面为本地包的生成方法参考：

用 Rust 创建一个 lib 项目。cargo.toml 需要包含类似于下面的内容。

```toml
[lib]
name="your_lib"
crate-type = ["cdylib"]
```

方法实现请参考下面的模板，使用时替换掉 T 和 R 即可

```rust
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern fn your_func<T,R>(para: T) -> R {
	// your logic
}
```

以下面的 `converter` 接口为例

```rust
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern fn order_should_pay(para: ConverterParameter) -> ConverterReturned {
	// your logic
}
```

## converter 接口形式

入参为[数据定义](data-define.md)里的 `ConverterParameter`, 出参为[数据定义](data-define.md)里的 `ConverterReturned`,

## convert_before 接口形式

入参为 `Instance`, 出参为  `Result<Instance>`，请见[数据定义](data-define.md)里的 `Instance`，出参的 json 形式如下：

1. 正常情况下请输出下面的内容，Ok 的值为改变后的 `instance` 对象，

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

## convert_after 接口形式

入参为`Vec<Instance>`, 出参为  `Result<Vec<Instance>>`，请见[数据定义](data-define.md)里的 `Instance`，出参的 json 形式如下：

1. 正常情况下请输出下面的内容，Ok 的值为改变后的 `instance` 对象数组。

   ```json
   {"Ok":[]}
   ```

2. 如果处理过程中遇到问题，请参考 `convert_before` 的问题处理方式

