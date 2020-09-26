# Executor接口

Executor 用于执行用户定制的逻辑，除了[内建 Executor](built-in.md) 和自动化的 Executor外，都需要用户按照给定的规范进行实现，Nature 目前有三种形式的 `Executor`：

- converter: 用于业务对象（`Meta`）间数据实例（`Instance`）的转换。
- convert_before: 用于业务对象转换前的预处理，如数据格式的修正，数据加载等。
- convert_after: 用于业务对象转换后的后置处理。

其实从功能上讲 `convert_before` 和 `convert_after` 完全可以用 `converter` 形式来替换但不建议，有下面的原因：

- `convert_before` 和 `convert_after` 一般只是技术上的处理，没有超出业务对象的范围，而 converter 则是衔接了两个业务对象。这个区别将来会用于业务对象关系的展示，既`convert_before` 和 `convert_after`是不会体现在这个界面上的。
- 从性能上讲 `converter` 与 `converter` 之间会有一个转换后的存储过程，而 `convert_before` ,`convert_after` 和 `converter` 之间是不需要存储的，效率会更高。

## 协议

目前 Nature 可以通过两种方式来调用`Executor`  ： http 和 localRust。

### http|https协议实现方式

请用一种自己喜欢的语言来实现一个可处理POST请求的 Web 服务。并确保 Nature 可以按照`关系`中定义的`url`访问到这个服务地址。

### localRust协议实现方式

为本地 lib 包。用 Rust 创建一个 lib 项目。cargo.toml 需要包含类似于下面的内容。

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

以下面的 converter 接口为例

```rust
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern fn order_should_pay(para: ConverterParameter) -> ConverterReturned {
	// your logic
}
```

## converter 接口

入参为 [数据定义](data-define.md)的 `ConverterParameter`, 出参为 [数据定义](data-define.md)的 `ConverterReturned`,

## convert_before 接口形式

入参为 `Instance`, 出参为  `Result<Instance>`，请见[数据定义](data-define.md)的 Instance，出参的 json 形式如下：

1. 正常情况下请输出下面的内容，Ok 的值为改变后的 instance 对象，

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

入参为`Vec<Instance>`, 出参为  `Result<Vec<Instance>>`，请见[数据定义](data-define.md)的 `Instance，出参的 json 形式如下：

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

## 示例

本文档中所涉及到所有内容都可以在 [Nature-Demo](https://github.com/llxxbb/Nature-Demo) 项目中找到对应的示例。

