# Delivery

Now we need some express companies to help us to transfer the goods to the customs, we want Nature to record the waybill info and query them at some time later, such as to finish the payment with express company.

The problem is that we want to query express info by waybill id, and we do not want to create a table outside of Nature to hold **"company id + waybill id"** and converter it to an **unique id**. Let's see how Nature to face on it.

## Define `meta`

```mysql
INSERT INTO meta
(full_key, description, version, states, fields, config)
VALUES('B:third/waybill', 'waybill', 1, '', '', '{}');
```

## Define converter

```mysql
-- orderState:outbound --> waybill
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'B:third/waybill:1', '{"selector":{"source_state_include":["outbound"]}, "executor":[{"protocol":"localRust","url":"nature_demo:go_express"}]}');

-- waybill --> orderState:dispatching
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:third/waybill:1', 'B:sale/orderState:1', '{"target_states":{"add":["dispatching"]}}');
```

## Converter Implement

```rust
#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
pub extern fn go_express(para: &ConverterParameter) -> ConverterReturned {
    // "any one" will be correct by Nature after returned
    let mut ins = Instance::new("any one").unwrap();
    ins.sys_context.insert("target.id".to_owned(), para.from.id.to_string());
    // ... some code to  submit package info to the express company,
    // ... and wait it to return an id.
    // the follow line simulate the express company name and the waybill id returned
    ins.para = "/ems/".to_owned() + &generate_id(&para.master.clone().unwrap().data).unwrap().to_string();
    // return the waybill
    ConverterReturned::Instances(vec![ins])
}
```

### Nature key points

`Instance.para`： here we set `Instance.para` property, this will hold **"company id + waybill id"** for you. at same time the `Instance.id` property will be set to 0, so that you can search this `Instance` just only by `para`.

`sys.target`: once again we used this in context, this time we used it in `converter`. but there is a bit queer, the target `meta` is `waybill` ,  why we need it here?. The reason is that **waybill --> orderState:dispatching** is an auto converter, that is Nature need to know which `orderState` will be updated. but the `waybill` can not tell it,  so must get it from `context`.

