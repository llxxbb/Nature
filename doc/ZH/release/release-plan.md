# 计划发布的版本

## Release 0.2

- 区分系统上下文和业务上下文
- 快速入门示例程序
- 执行器
  - 状态`Instance`支持`or`条件
  - 使用 `min-max-heap` 来支持优先级
  - 可以通过 contex 来 申请ID
- 数据库：mysql 支持 r2d2

* `Meta`：支持属性验证，更新时验证是否有`Relation`在使用，如有则发出警告
* ID 生成器 snowflake for 128 bit implement.
* 支持第三方插件
  * monitor point
  * traffic limit
* 后台管理界面：更新配置时能够更新多个实例上的缓存
* 密文存储：简单方案，不区分用户，对称加密，公私钥方案见云版
* 提供公用外部系统查询接口：thirdQuery

## 有待确认

