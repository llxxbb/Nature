# 计划发布的版本

计划的总体原则：
- 功能性先于非功能性需求
- 应用情景支持，支持内部优先支持外部（如网关）

## Release 0.1.0

- 文档:丰富业务场景，补充论坛。
- 内置执行器
  - 内置执行器有自己的配置
  - 实现`计数器`内置执行器。 

## Release 0.1.1

- `关系`配置支持`or`条件，可应用于状态选择和上下文选择。
- 以react的方式实现对restful`执行器`的调用.以消除下面的警告
[2020-03-15][21:34:35][reqwest::wait][WARN] blocking API used inside an async Executor can negatively impact perfomance
  
## Release 0.2.0

- 行使网关只能，充当所以业务系统的入口。
  - 查询外系统接口：queryThird
- 读取优化
  - Executor 支持内置缓存
  - 提供一致性哈希，以分散缓存的压力

- 支持第三方插件
  * monitor point
  * traffic limit

## Release 0.3.0

- 执行器：使用 `min-max-heap` 来支持优先级
- 自路由：对`执行器`返回的自路由进行支持。
- 数据库：mysql 支持 r2d2
- ID 生成器 snowflake for 128 bit implement.


## Release 1.0.0

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


