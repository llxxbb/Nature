# 计划发布的版本

发布原则：Demo 项目，单元测试都通过 计划的总体原则：

- 功能性先于非功能性需求
- 应用情景支持，内部优先于外部（如网关）

下面为正在开发的内容

Release 1.4.0

- Output `Instance` for `MetaType:Null`
- management：support failed task query
- optimize: simplify Retry and make the config clear
- 
### It should commit

### 未完成

bug fix: insert error task to task table already exists.
测试异常的任务

optimize: cancel run the same task when it has been finished.
