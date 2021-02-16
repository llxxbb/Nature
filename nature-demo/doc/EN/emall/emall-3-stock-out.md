# stock-out

When the order was paid we should carry out the contract. The first step is stock-out. But we suppose that the warehouse system is old and slow, and that would cause timeout, so we need another mechanism to resolve the problem: callback.

## Some limited

In real conditions, an order's may include variant goods, these goods may involves many warehouses,  and each of them need to be traced separately. I don't want to make this chapter too complex, so I suppose there is only one warehouse can be used.

Another thing is, a warehouse process `stock-out-application` instead of `order` in usually. To simplify this demo  let's suppose the warehouse system is already exists before Nature and can process business by `order` info, so we need not to define `meta` for warehouse.

## Define `converter`

```mysql
-- orderState:paid --> orderState:package
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'B:sale/orderState:1', '{"selector":{"source_state_include":["paid"]},"executor":[{"protocol":"http","url":"http://localhost:8082/send_to_warehouse"}],"target_states":{"add":["package"]}}');
```

### Nature key points

`Protocol::http`: Nature can post a request to a restful implement converter.

## The process flow

```mermaid
graph LR
	order:paid-->send[send to warehouse]
	send-->wh[warehouse process]
	wh-->order:outbound	
```

## Implement the converter

The main code is list below:

```rust
fn send_to_warehouse(para: Json<ConverterParameter>) -> HttpResponse {
    thread::spawn(move || send_to_warehouse_thread(para.0));
    // wait 60 seconds to simulate the process of warehouse business.
    HttpResponse::Ok().json(ConverterReturned::Delay(60))
}

fn send_to_warehouse_thread(para: ConverterParameter) {
    // wait 50ms
    thread::sleep(Duration::new(0, 50000));
    // send result to Nature
    let rtn = DelayedInstances {
        task_id: para.task_id,
        result: ConverterReturned::Instances(vec![para.from]),
    };
    let rtn = CLIENT.post(&*NATURE_CALLBACK_ADDRESS).json(&rtn).send();
    let text: String = rtn.unwrap().text().unwrap();
    if text.contains("Err") {
        error!("{}", text);
    } else {
        debug!("warehouse business processed!")
    }
}
```

### Nature key points

`callback`: `converter` can processed asynchronously for a long-time-task, in this situation converter need return immediately with `ConverterReturned::Delay(seconds)` , this tell Nature the `converter` will return the real result before the **seconds** passed, if not Nature will try again.

Another point is the real result `converter` returned must be `DelayedInstances` but not `ConverterReturned`. And the  `DelayedInstances.task_id` must be  `para.task_id`, this will tell Nature to restore the unfinished task and go on.

## Give outbound info to Nature

Now the warehouse packaged the goods and make it outbound, and then tell this info to Nature, so that Nature can driver the following steps to run.

```rust
	let last = wait_for_packaged(order_id);    
	let mut instance = Instance::new("/sale/orderState").unwrap();
    instance.id = last.id;
    instance.state_version = last.state_version + 1;
    instance.states.clear();
    instance.states.insert("outbound".to_string());
    let rtn = send_instance(&instance);
```

### Nature key points

Like normal input to Nature, but here you must use `last`'s id, otherwise Nature will generate one for you,  then your `order` will not find it's outbound info anymore.

`state_version` must add one, otherwise it will conflict.