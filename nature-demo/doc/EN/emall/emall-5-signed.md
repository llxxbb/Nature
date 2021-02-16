## Signed

This is the last step, user receive goods and sign the waybill, but express company will not give the signed info to our system, so we need the custom login to our system and signed it manually. but many of them do not do that at all, how do we to accomplish it? An idea is we will wait fortnight, when there is no complaint, we will signed it automatically.

For our benefit, we make fortnight to 1 seconds, so that you can see the result quickly.

## Define `meta`

```mysql
INSERT INTO meta
(full_key, description, version, states, fields, config)
VALUES('B:sale/orderSign', 'order finished', 1, '', '', '{}');
```

## Define converter

```mysql
-- orderState:dispatching --> orderSign
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'B:sale/orderSign:1', '{"delay":1,"selector":{"source_state_include":["dispatching"]}, "executor":[{"protocol":"localRust","url":"nature_demo:auto_sign"}]}');

-- orderSign --> orderState:signed
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderSign:1', 'B:sale/orderState:1', '{"target_states":{"add":["signed"]}}');
```

### Nature key points

`delay`: will tell Nature execute this conversion after appointed time.  notice that,  delayed task only can be picked up by `Nature-Retry` project , so you need to start it up.