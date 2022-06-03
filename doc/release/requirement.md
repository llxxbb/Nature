# 有待确认的需求
    
- remove reqwest dependence, but actix_client not safe for multi-thread, need new for every time
- use lru-cache for cache

- 修正 prepare.md
- 流式计算：
  map, filter, sum, fold, groupBy, reduce.


## 


## Release 3.0
- 自路由：对`执行器`返回的自路由进行支持。

- 行使网关职能，充当所以业务系统的入口。
  - 查询接口，支持查询外系统接口：queryThird
  - Demo : 获取平均分（）
  - 读取优化
    - Executor 支持内置缓存 适用于网关
    - 提供一致性哈希，以分散缓存的压力
    
- 支持第三方插件
  * monitor point
  * traffic limit

## Release 3.0

- 分布式重试服务
- 文档：
    - 英文文档
    - `Instance` 的 `context` 说明。
    - 对 《概念》画一个流程图以说明怎样生成`Instance`
