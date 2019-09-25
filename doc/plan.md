# plan

## finished but did not submit

## Underway

    MetaCacheGetter return `Meta` 
    inplement From<Meta> and TryInto<Meta> for RawMeta.
    status field is empty but the meta is status.
    
    generate order
        
## meta

`instance` parametrization (pending)
    可以用于条件查询，如 订单-> 用户订单 关系中，订单不能按用户维度检索订单，
    但在 用户订单 的 instance 中 引入userid parameter 用于区分用户。
    related works：
        - architecture doc
        - demo dock
        - implement

## add synchronous call for outer( the main part of the gateway)

## instance lifetime
    different meta different lifetime and different strategy.

## distribute deploy retry system

## demo
    shopping cart(add, remove, clear, submit)

