# 计划发布的版本

发布原则：Demo 项目，单元测试都通过 计划的总体原则：

- 功能性先于非功能性需求
- 应用情景支持，内部优先于外部（如网关）

下面为正在开发的内容

Release 1.6.0

### It should commit

### 未完成

check unused package

使用它 Rwlock 取代缓存的 Mutex

优化状态数据处理
优化 TaskForConvert
优化 RawTask
引入 snowflake id 生成器
为META 和 relation 增加 id 字段。

- 测试异常的任务

optimize: cancel run the same task when it has been finished.

文档：工作机制
对异常数据的处理机制

检查 channel_convert 是不是足够轻量，因为它是第一个接受者。可考虑多个线程进行接收。