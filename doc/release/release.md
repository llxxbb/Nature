# 发布的功能

## Release 0.22.3 2021-02-14

- bug fix: get_by_key_range
- optimize `KeyCondition`

## Release 0.22.2 2021-02-13

- Optimize: support recent `Instance` for management UI.
- bug fix: query `Instance` by id
- Optimize: query `Instance` by key

## Release 0.22.1

- manager interface: query downstream for `Instance`

## Release 0.22.0

- Manager: add `Instance` interface for UI management
- change `Instance` id from u64, so that js long will be work fine

## Release 0.21.1 2021-01-23

change manage client port to 8280 to avoid conflict with nature.exe

## Release 0.21.0 2021-01-16

- add manager backend(will be used by Nature-Manager-UI)
- db:meta add id field
- db:relation add id field

## Release 0.20.3 2020-11-21

English doc: architecture.md and others

## Release 0.20.2 2020-11-01

Chinese doc: fix architecture.md

## Release 0.20.1 2020-10-11

- finished english doc and adjust chinese doc:
  - meta.md
  - executor.md
  - nature-interface.md
  - data-define.md
  - builtin-executor

## Release 0.20 2020-10-04

- incompatible change: change `Instance` table design, split `ins_key` into `meta`, `ins_id` and `para`
- incompatible change: use decimal replace hexadecimal to represent the instance.id
- remove u128 support
- incompatible change: KeyCondition.id type changed to u64 from String
- incompatible change: move need_all, need_any, need_none from TargetState to FlowSelector 
- incompatible change: merge TargetState to RelationTarget
- incompatible change: context_name to dynamic_para
- simplify the readme(CH && EN)
- bug fix: if target last instance check failed, should return environment error 

## Release 0.19 2020-09-26

- make ConverterReturned to fit for json communication
- simple readme.md(CH)
- add Executor.md(CH)
- add nature-interface.md(CH)

## Release 0.18 2020-09-20

- incompatible change: task_id from char to bigint
- incompatible change: filter_before -> convert_before
- incompatible change: filter_after -> convert_after

## Release 0.17 2020-09-13

- change input method return type from u64 to hex string
- change crate `fern` to `env_logger`
- optimize input interface performance

## Release 0.16.0 2020-09-06

- merge nature, common, db and retry projects together
- merge demo, demo-common, demo-executor, demo-executor-restful together.

## Release 0.15.0

- id64 support: use u64 as generated id
- user can custom log level
- optimize: normalize meta property for inputted instances
- optimize: message for the database when it is unready
- bug: generated state-instance create_time does not set

## Release 0.14.1

- rename MetaSetting.output_last to MetaSetting.only_one and fix bug around
- optimize: builtin-task_checker include update time check

## Release 0.14 2020-08-16

- MetaSetting: add `output_last` property. only used by `MetaType::Loop`, output the instance only when loop finished. This requires the multi_meta has only one item.
- add pre-filter-builtin: task-checker: make sure the certain tasks are all done before execute a `Executor`
- add pre-filter-builtin: para_as_key: convert para part as content key and old content as value
- builtin-executor:merge 
  - support top
  - rename Setting.key.VecTuple to Setting.key.Content. 
- bug fix: meta load verify error 
- Optimize and bug fix: MetaType::Loop
- bug fix: exchange the order of generate ID and Para
- Optimize: make builtin:loader setting.time_part optional
- fix bug: retry is over the max times should move to `task_error` table

## Release 0.13 2020-08-08

- sys_context: para.dynamic property supported
- RelationTarget
  - rename from copy_para to append_para, and add append support
  - add property: context_name to form sys_context:para.dynamic
- bug fix: break task process when task repeated 
- builtin-merge: add KeyType::None support

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
- bug fix: MetaType::Multi has none from_key generated
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
- optimize: fetch tasks which need to redo
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

