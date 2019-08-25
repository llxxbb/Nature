# plan

## finished but did not submit

## Underway
       
    generate order
    
    meta support state
    instance support state
    status:
        can include sub status
        satus can mutex each other
        
## meta
    status field is empty but the meta is status.

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

