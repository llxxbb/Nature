# Converter Of Local Rust Implement 

Tell you how to implement a Rust Converter locally

## Cargo.toml

```toml
[dependencies]
nature_common = {path = "../Nature-Common"}

[lib]
crate-type = ["dylib"]
```

## lib.rs

```rust
extern crate nature_common;

use nature_common::*;


#[no_mangle]
pub extern fn simple_convert(para: CallOutParameter) -> ConverterReturned {
    // TODO your logical
    ConverterReturned::Instances(vec![your_instance])
}

```