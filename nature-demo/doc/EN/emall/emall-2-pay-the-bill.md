# Pay for the bill

Now the user will pay for the order.  Here we make it a little complex,  we suppose any one of the user's card is not enough to pay for the bill, but the total of three of them is ok. Let's define the business logic.

 ## Define `meta`

```mysql
INSERT INTO meta
(full_key, description, version, states, fields, config)
VALUES('/B/finance/payment', 'order payment', 1, '', '', '{}');

INSERT INTO meta
(full_key, description, version, states, fields, config)
VALUES('/B/finance/orderAccount', 'order account', 1, 'unpaid|partial|paid', '', '{"master":"/B/sale/order:1"}');
```

The `payment` will record the user each pay info. 

The `orderAccount` is used to mark the order pay state. It's also a state `meta`.

## Define `converter`

```mysql
-- order --> orderAccount
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('/B/sale/order:1', '/B/finance/orderAccount:1', '{"executor":[{"protocol":"localRust","url":"nature_demo:order_receivable"}],"target_states":{"add":["unpaid"]}}');

-- payment --> orderAccount
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('/B/finance/payment:1', '/B/finance/orderAccount:1', '{"executor":[{"protocol":"localRust","url":"nature_demo:pay_count"}]}');

-- orderAccount --> orderState
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('/B/finance/orderAccount:1', '/B/sale/orderState:1', '{"selector":{"source_state_include":["paid"]},"target_states":{"add":["paid"]}}');
```

There we need several converters outside of Nature to accomplish our task:

**order --> orderAccount** is used to create an account for each order and record the receivable info.

**payment --> orderAccount**  records each pay for the order from `payment`.

### Nature key points

 The `executor` node in `settings` describing the outside converter that we will used. let's see some properties of it:

| field    | value description                                            |
| -------- | ------------------------------------------------------------ |
| protocol | how to communicate with the executor: `LocalRust` or `http`, to simplify this demo, we use `LocalRust` |
| url      | where to find the executor                                   |

`source_state_include`: it is a filter, only `orderAccount`'s state include "paid" state that the converter can be run.

**orderAccount --> orderState** is a `auto-converter`, because there is no `executor` is defined. this is like "order --> orderState" in the previous chapter.

## Define business objects

In project `Nature-Demo-Common` we need define some business entities which would be used in `Nature-Demo` and `Nature-Demo-Converter`.

```rust
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Payment {
    pub order: u128,
    pub from_account: String,
    pub paid: u32,
    pub pay_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct OrderAccount {
    pub receivable: u32,
    /// can not be over the receivable, the extra money would be record to the field `diff`
    /// design in this way can hold each pay which is over
    pub total_paid: u32,
    pub last_paid: u32,
    /// record the reason for account change
    pub reason: OrderAccountReason,
    /// positive: over paid, negative : debt
    pub diff: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum OrderAccountReason {
    NewOrder,
    Pay,
    CancelOrder,
}

impl Default for OrderAccountReason {
    fn default() -> Self {
        OrderAccountReason::Pay
    }
}
```

## Implement converter for  "**order --> orderAccount**"

In project `Nature-Demo-Converter` we implement it like follow:

```rust
#[no_mangle]
pub extern fn order_receivable(para: &ConverterParameter) -> ConverterReturned {
    let order: Order = serde_json::from_str(&para.from.content).unwrap();
    let oa = OrderAccount {
        receivable: order.price,
        total_paid: 0,
        last_paid: 0,
        reason: OrderAccountReason::NewOrder,
        diff: 0 - order.price as i32,
    };
    let mut instance = Instance::default();
    instance.content = serde_json::to_string(&oa).unwrap();
    ConverterReturned::Instances(vec![instance])
}
```

There is no secret in this implement, but you should know [how to implement a local converter](https://github.com/llxxbb/Nature/blob/master/doc/help/howto_localRustConverter.md).

### Nature key points

You can get your business-object through:

```rust
rustserde_json::from_str(&para.from.content).unwrap();
```

You should put your business-object to `Instance.content` field for returning.

You can return only one `instance` for state `Meta` like `orderAccount`

## Implement converter for  "**payment --> orderAccount**"

```rust
#[no_mangle]
pub extern fn pay_count(para: &ConverterParameter) -> ConverterReturned {
    let payment: Payment = serde_json::from_str(&para.from.content).unwrap();
    if para.last_state.is_none(){
        return ConverterReturned::EnvError;
    }
    let old = para.last_state.as_ref().unwrap();
    let mut oa: OrderAccount = serde_json::from_str(&old.content).unwrap();
    let mut state = String::new();
    if payment.paid > 0 {
        state = "partial".to_string();
    }
    oa.total_paid += payment.paid;
    oa.diff = oa.total_paid as i32 - oa.receivable as i32;
    if oa.diff > 0 {
        oa.total_paid = oa.receivable;
    }
    if oa.diff == 0 {
        state = "paid".to_string();
    }
    oa.last_paid = payment.paid;
    oa.reason = OrderAccountReason::Pay;
    let mut instance = Instance::default();
    instance.content = serde_json::to_string(&oa).unwrap();
    instance.states.insert(state);
    ConverterReturned::Instances(vec![instance])
}
```

### Nature key points

When `orderAccount` not initialized, we should return`ConverterReturned::EnvError`,  Nature will retry later.

Except you can get `Payment` from `&para.from.content`, you can get last `orderAccount` from:

```rust
    let old = para.last_state.as_ref().unwrap();
    let mut oa: OrderAccount = serde_json::from_str(&old.content).unwrap();
```

When you return a new `orderAccount`, Nature will increase it's `state_version` automatically in the backend. **You don't worry about the concurrent problem,  when this event occurred Nature will retry it again**.

### Question

This converter would modify the last `orderAccount` and return the modified, but Nature how to find the last `orderAccount`?  The explanation please see the following section.

## Submit payment date to Nature

You will see the whole codes in project `Nature-Demo`,  key codes list here only:

```rust
pub fn user_pay(order_id: u128) {
    let _first = pay(order_id, 100, "a", Local::now().timestamp_millis());
    let time = Local::now().timestamp_millis();
    let _second = pay(order_id, 200, "b", time);
    let _third = pay(order_id, 700, "c", Local::now().timestamp_millis());
    let _second_repeat = pay(order_id, 200, "b", time);
}

fn pay(id: u128, num: u32, account: &str, time: i64) -> u128 {
    let payment = Payment {
        order: id,
        from_account: account.to_string(),
        paid: num,
        pay_time: time,
    };
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("sys.target".to_string(), id.to_string());
    match send_instance_with_context("/finance/payment", &payment, &context) {
        Ok(id) => id,
        _ => 0
    }
}
```

### Nature key points

Are you remember the question above? the secret is **"sys.target"** of instance's context! That indicate which `orderAccount` would be load.

## Different with traditional development

We finished the complex logic by use about 100 lines of code, include concurrent, state version conflict control and retry policy etcetera, it's very hard for traditional development mode.

You can see, we add `orderAccount` without modify already exists logic for `order` which is in previous chapter, this is impossible for traditional mode, that means Nature will make your work pluggable, extensible and easy to maintain.

You will never mind `orderAccount`'s state_version is what and each change how to go,  they are trivial mater for Nature to take care of.

More importantly there is no logical code written  for `orderState`, but you can see a version 2 of it lying in the database table, and it's state changed to "paid" automatically. It was not the developer's work any more, product manager just do it well.