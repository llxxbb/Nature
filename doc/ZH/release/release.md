
# 发布的功能

## Release 0.12 2020-08-03

- add `MetaType::Loop` support
- MetaType::Loop & MetaType::Multi can't be used as `from` in relation
- enhance meta_cache
  - set state to false for Multi
  - set state to true for Loop
- enhance MetaSetting
- builtin-loader:
  - set instance.sys_context with `loop.next`
  - gen next loop from `from.sys_context`
- builtin-merge: rename from sum to `merge` and support batch mode.
- builtin-time_range: 
  - para_part -> time_part
- fix bug for instance_dao::get_by_key_range
- fix bug for task_dao::raw_to_error
- fix bug: timeZone problem when load `Instance`
- bug fix: MetaType::Multi has no from_key generated
- optimize: short time expression for time_range

## Release 0.11 2020-07-26

- add built-in filter support
- new built-in filter: loader. It can load data from Instance Table.
- fix bug: built-in executor: time_range: interval calculate error.
- incompatible change: get_by_meta changed to get_by_key_range
- enhance keyCondition
- enhance instance_dao

## Release 0.10 2020-06-27

- built-in executor enhanced for `sum`: support five modes old,new,min,max,sum 
- prevent sta_version skipping to avoid some invalid input
- fix bug: copy_para does not work for state-target
- fix bug: duplicate input status-instance cause error
- mysql_async::Error::io: changed from LogicalError to EnvError
- demo: multi-delivery

## Release 0.9

- relation setting: add id_bridge property
- fix bug: for check_id
- enhanced Protocol::Auto
- incompatible change: relation setting: rename upstream_para to copy_para

## Release 0.8 2020-06-08

- replace diesel with mysql_async, because of libmysqlclient_sys cannot compile under stable rust toolchain for win10
- add `RelationSetting.delay_on_para`
- add interface `query_by_meta` for outside call
- incompatible change: change `Instance` table design
- incompatible change: change `task` table design
- incompatible change: `conflict_avoid` rename to `cache_saved` 
- incompatible change: `DelayedInstances` and `ConverterParameter`
- incompatible change: rename `ParaForIDAndFrom` to `IDAndFrom`
- incompatible change: rename `QueryByID` to `KeyCondition`
- incompatible change: builtin-executor dimensionSplit renamed to scatter and make it clearer for its usage
- incompatible change: target.id changed from decimal to hexadecimal
- builtin executor: timer
- bug fixed: cache_saved is invalid
- bug fixed: create_time unset for `MetaType::Multi`
- bug fixed: task clean time is invalid 

## Release 0.7 2020-4-30

- builtin-executor sum
- Relation Setting: add `target.upstream_para` support
- incompatible change: target_states changed to target.state for relation-setting. 
- incompatible change: original `executor.filter` will rename to `executor.filter_after`
- add `executor.filter_before` support.
- bug fix: state instance deep conflicts when save will lead to actix-rt overflow its stack
- bug fix: task missing problem due to id generation has some problem
- bug fix: none-end loop, by add conflict version to `convert-task` to avoid it
- bug fix: should break process when filter_after run into error;

## Release 0.6 2020-4-13

- multi-Executor is replaced with single-Executor for relation.
- add filter support for executor 
- add r2d2 support for mysql.

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

