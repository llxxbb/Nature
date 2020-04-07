
# 发布的功能

## Release 0.5 2020-4-6

- support multi-meta output
- add MetaSettingTemp for MetaSetting to improve the performance for multi-meta checking
- optimize RawTask storage size
- support built-in executor
- add support for executor self setting
- add builtin executor: dimensionSplit

## Release 0.4.0 2020-4-2

- update actix-web to 2.0 version
- use async replace actor to simplify code
- execute `Executor` asynchronously 

## Release 0.3.0 2020-3-29

- merge `parallel task`, `plan` and `Sequential` to `batch`
- fix bug: parallel input cause state version error increment

## Release 0.2.0 2020-3-26

- task will be remained after executed,
- delete executed task after a given delay
- bug fix: conflict with same source cause error task generated 
- bug fix: retry module update times and time failed.  

## Release 0.1.0 2020-3-25

- instance table add sys_context field
- `Relation`support [any} option
- system context : sys.target -> target.id
- optimize: fetch task witch need to redo
- change mysql as default db because of sqlite's cb_lock
- update reqwest to 0.10
- bug fix: get old plan error “not 1 or 0 but get 2”

## release 0.0.2 2020-3-15

- 支持的数据库：sqlite, mysql
- 支持的流程编排方式：静态，动态
- 支持的执行器协议：restful, rust 本地库
- 支持回调接口：`执行器`可长时间执行任务，并通过回调接口返回结果
- 任务派发前要落盘，以防止`Meta`和`Relation`发生变化引起数据不一致。
- 支持业务自定义参数作为主键的一部分，这有利于和外部项目集成。

