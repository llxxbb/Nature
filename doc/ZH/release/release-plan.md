# 计划发布的版本

发布原则：Demo 项目，单元测试都通过
计划的总体原则：
- 功能性先于非功能性需求
- 应用情景支持，内部优先于外部（如网关）

## Release 0.14.2

- optimize: normalize meta property for inputted instances

### should commit


### 未完成

- bug: auto generated instance create_time does not set

- user can custom log level, web log level

- 数据库
{"Err":{"EnvironmentError":"Input/output error: 由于目标计算机积极拒绝，无法连接。 (os error 10061)"}}

- 极简编程
  - 融合这么多特性，最终目的就是简化编程。

- concurrent tasks limit and monitor 
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
- ID 生成器 snowflake for 128 bits implement.
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

