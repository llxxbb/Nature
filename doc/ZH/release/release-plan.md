# 计划发布的版本

发布原则：Demo 项目，单元测试都通过
计划的总体原则：
- 功能性先于非功能性需求
- 应用情景支持，内部优先于外部（如网关）

## Release 0.14

- MetaSetting: add `output_last` property. only used by `MetaType::Loop`, output the instance only when loop finished. This requires the multi_meta has only one item.
- add builtin:task-checker: make sure the certain tasks are all done before execute a `Executor`
- builtin:merge support top
- bug fix: meta load verify error 
- Optimize and bug fix: MetaType::Loop
- bug fix: exchange the order of generate ID and Para
- Optimize: make builtin:loader setting.time_part optional
- fix bug: retry is over the max times should move to `task_error` table

### should commit



### 未完成

- verify loop : per-batch 2 records
  - output_last : false
  - can't start second loop

- demo : sale-statistics

- 修正，订单->区间统计任务
    minute : consistent with second data 
    - 将分钟数据转移到其他小节中去。

- doc: instance-loader

- concurrent task number limit 
- redo check busy first

- 压测

## Release 1.0.0

- 行使网关只能，充当所以业务系统的入口。
  - 查询外系统接口：queryThird
  - Demo : 获取平均分（）
- 读取优化
  - Executor 支持内置缓存 适用于网关
  - 提供一致性哈希，以分散缓存的压力

- 支持第三方插件
  * monitor point
  * traffic limit

## Release 1.1.0


- 自路由：对`执行器`返回的自路由进行支持。
- ID 生成器 snowflake for 128 bit implement.
- builtin-executor sum :
  - for stream sum

## Release 2.0

- `Meta`：不同的`Meta` 的 `Instance` 有不同的生命周期
- `Meta`：支持属性验证，更新时验证是否有`Relation`在使用，如有则发出警告

## Release 3.0

- 提供后台管理界面
  - 更新配置时能够更新多个实例上的缓存
- `Meta`：支持属性验证，更新时验证是否有`Relation`在使用，如有则发出警告
- `Meta`：不同的`Meta` 的 `Instance` 有不同的生命周期
-  密文存储：简单方案，不区分用户，对称加密，公私钥方案见云版
- 分布式重试服务
- 文档：
  - 英文文档
  - `Instance` 的 `context` 说明。
  - 对 《概念》画一个流程图以说明怎样生成`Instance`
- 执行重试任务时，检测是否忙碌，如果忙碌，返回忙碌信息。

