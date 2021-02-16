# Executor

Except for [built-in Executor](built-in.md) and automated `Executor`, you need to implement and deploy `Executor` according to the interface protocol given by Nature. `Executor` must be defined in [Relation](relation.md) and it will be called by Nature. Nature currently has three forms of `Executor`:

- converter: Used to convert `Instance` between [Meta](meta.md).
- convert_before: Used for preprocessing before the `Instance` conversion, such as data format modification, data loading, etc.
- convert_after: Used for post-processing the `Instance`s after conversion.

The three forms of `Executor` can all be found in the [Nature-Demo](../../../nature-demo/README_EN.md) project.

In fact, functionally speaking, `convert_before` and `convert_after` can be replaced by `converter` form, but Nature does not recommend, for the following reasons:

- `convert_before` or `convert_after` is generally only technical processing, and do not exceed the scope of the business object, while `converter` connects two business objects, which is beyond the scope of the business object. On the management interface, `converter` will be used to display business object relationships in the future, but `convert_before` and `convert_after` will not be displayed directly.
- for the benefit of the performance: `converter` have a process for writing to database, while `convert_before` and `convert_after` do not need storage, so more efficiently. this will be a greater impact on large massive data processing.

## Protocol

Currently, Nature can call `Executor` in two ways: http and localRust. Nature will supplement Other protocols in the future.

### http|https protocol implementation

Please use any language you like to implement a Web-Service can handle POST requests. Then make sure Nature can access this service address via the `url` defined in [Relation](relation.md)'s `executor` property.

### localRust protocol implementation

For the local lib package, the generated package needs to be placed in the same directory as the nature executable file lived in. The following is a reference for the method to generation the lib package:

Create a lib project in Rust. cargo.toml needs to contain something like the following.

```toml
[lib]
name="your_lib"
crate-type = ["cdylib"]
```

For method implementation, please refer to the template below, replace T and R when using it.

```rust
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern fn your_func<T,R>(para: T) -> R {
	// your logic
}
```

Take the `converter` interface as an example

```rust
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern fn order_should_pay(para: ConverterParameter) -> ConverterReturned {
	// your logic
}
```

## converter interface form

The input parameter is `ConverterParameter` in [Data Definition](data-define.md), and the output parameter is `ConverterReturned` in [Data Definition](data-define.md),

## convert_before interface form

The input parameter is `Instance`, and the output parameter is `Result<Instance>`. Please see `Instance` in [Data Definition](data-define.md). The json format of the output parameter shown as follows:

1. if ok, please output the following content. The value of Ok is the changed `instance` object.

    ```json
    {"Ok":{}}
    ```

2. If you encounter problems during processing, the following content you should return:

    ```json
    {"Err":{"LogicalError":"err message"}}
    ```

    or

    ```json
    {"Err":{"EnvironmentError":"err message"}}
    ```

## convert_after interface form

The input parameter is `Vec<Instance>`, and the output parameter is `Result<Vec<Instance>>`. Please refer to `Instance` in [Data Definition](data-define.md). The json format of the output parameter shown as follows:

1. if ok, please output the following content, the value of Ok is the changed `instance`s array.

    ```json
    {"Ok":[]}
    ```

2. If you encounter problems during processing, please refer to `convert_before`