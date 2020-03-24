# 使用建议

- 避免构成死循环，

- 尽可能的使用关系的 use_upstream_id 设置来代替系统上下文 tagget.id。既配置优于编程。

- `Meta` 的 conflict_avoid 使用得当会提升性能。这会提升重复生成实例的情况。如“统计计划”`Meta`
