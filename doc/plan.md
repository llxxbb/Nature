# plan


refine architecture of doc

`thing` rename to `meta`

change table define:
    `thing_defines` : add config, such encrypt, is status
    `one_step_flow` : thing changes to meta
    `instances` : thing changes to meta
    `task` : thing changes to meta
    `task_error` : thing changes to meta
    `plan` : thing changes to meta

`instance` parametrization (pending)
    可以用于条件查询，如 订单-> 用户订单 关系中，订单不能按用户维度检索订单，
    但在 用户订单 的 instance 中 引入userid parameter 用于区分用户。
    related works：
        - architecture doc
        - demo dock
        - implement

convention
    `meta` define: /B /S /NULL /D
    

add synchronous call for outer( the main part of the gateway)

root

distribute deploy retry system

- Implement a demo



