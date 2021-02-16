# Statistics

After paid we want to make statistics for the products, and analysis them by multi-dimensions, but we are lazy to writing the code. Luckily Nature can do that for you.

## Define `meta`

```mysql
INSERT INTO meta
(full_key, description, version, states, fields, config)
VALUES('M:statistics/orderTask', 'total sold every hour', 1, '', '', '{"multi_meta":{"keys":["minute","hour"]}, "conflict_avoid": true}');
```

### how to make statistics

If we we increase the counter for every order use `state-instance`, there would be many conflicts for high parallel process, and another question is that we would generated great volume of `state-instace`, so it's a terrible thing. 

There is a way to do it is that we count it every minute for minute data and every hour for hour data. to do that we should generate one none state task-instance for every minute and one for every hour. 

### Nature key points

**"M"** `metaType` : express `multi-meta ` which will be processed parallelly, each key is defined in the `multi_meta.keys` property. For this demo, after converted Nature will save two instances.

```
B:statistics/orderTask/minute with para: current minute
B:statistics/orderTask/hour with para: current hour
```

**"conflict_avoid"** setting tell Nature that the same instances will generated many times and Nature should cache it and check it befor save. If `false`(default) is set would lead to a large number of duplicated insertions. so the performance would be very bad.

## Define converter

```mysql
-- orderState:paid --> task
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'M:statistics/orderTask:1', '{"selector":{"source_state_include":["paid"]},"executor":[{"protocol":"localRust","url":"nature_demo:statistics_task"}]}');
```



## unready

why delay 70 seconds? 

## 

### Questions

There is a question, how to identify each inputted data for `consume/input`? used Nature generated instance id? no, it's hard to query it out, so we use parameterize instance technology in this converter.

update the stateful-counter is a big bottleneck problem for busy system,  so we use Nature's `delay` technology and stateless `meta` to hold every past minute data. You can form you hour data, day data and any wide range data through this mechanism, but in this demo we stopped at minute data, It's enough for you to understand how to use Nature for statistics effectively.

### Nature key points

Another question is how to give multi-dimensions info to the following converter?,  sealed it to the `Instance.content` property? This is not a good idea, because `content`'s structure must be resolved by code! that is not we wanted. `context` will face on this problem. here we just used them in converter settings, no coding! (of course you can use `context` in your code explicitly).



```mysql
-- orderSign --> orderState:signed
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:statistics/consume/input:1', 'B:statistics/consume/product/total/minute:1', '{"target_states":{"add":["signed"]}}');
```

