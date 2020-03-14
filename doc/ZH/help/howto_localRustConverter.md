# Executor Of Local Rust Implement 

Tell you how to implement a Rust Executor locally

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
pub extern fn simple_convert(para: ConverterParameter) -> ConverterReturned {
    let obj: YouBusinessObject = serde_json::from_str(&para.from.content).unwrap();
    // TODO your logical
    let mut instance = Instance::default();
    instance.content = serde_json::to_string(&new_other_business_object).unwrap();
    ConverterReturned::Instances(vec![instance])
}

```

You can get your business-object through:

```rust
serde_json::from_str(&para.from.content).unwrap();
```

You should put your business-object to `Instance.content` field.