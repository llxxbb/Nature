# 计划发布的版本

发布原则：Demo 项目，单元测试都通过 计划的总体原则：

- 功能性先于非功能性需求
- 应用情景支持，内部优先于外部（如网关）

下面为正在开发的内容

Release 1.5.0

- update to 2021 rust edition
- use async_channel to replace the rust native channel.
- use Local-Time for logger
- 将 NatureError 和 Result 移动到 common 模块下。
- release 中包含 demo 所需的执行器和可执行文件
- 文档修复
  - Readme.md
  - doc/ZH/help/executor.md
  - nature-demo/doc/ZH/prepare.md
- 插件的位置可通过 .env 配置文件中的 PLUGIN_PATH 进行定义，或者在工作目录下查找。
- 去除对 sqlite 的支持
- Instance 增加 new_with_empty_meta()方法。
- 多库房示例项目: 修正数据重复问题。

### It should commit

### 未完成

修正 shell/Nature.jmx 中的问题
检查 channel_convert 是不是足够轻量，因为它是第一个接受者。可考虑多个线程进行接收。

优化状态数据处理
优化 TaskForConvert
优化 RawTask
引入 snowflake id 生成器
为META 和 relation 增加 id 字段。

- 测试异常的任务

optimize: cancel run the same task when it has been finished.

文档：工作机制
对异常数据的处理机制
