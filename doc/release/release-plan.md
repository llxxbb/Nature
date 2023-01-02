# 计划发布的版本

发布原则：Demo 项目，单元测试都通过 计划的总体原则：

- 功能性先于非功能性需求
- 应用情景支持，内部优先于外部（如网关）

下面为正在开发的内容

Release 1.5.0

- 将 NatureError 和 Result 移动到 common 模块下。
- release 中包含 demo 所需的执行器和可执行文件
- 文档修复
  - Readme.md
  - doc/ZH/help/executor.md
  - nature-demo/doc/ZH/prepare.md
- 规定将插件统一放到 plugin 目录下，可通过 .env 配置文件中的 PLUGIN_PATH 进行重定义。
- 去除对 sqlite 的支持
- Instance 增加 new_with_empty_meta()方法。
- 多库房示例项目: 修正数据重复问题。

### It should commit

### 未完成

引入 snowflake id 生成器

为META 和 relation 增加 id 字段。

- 测试异常的任务

optimize: cancel run the same task when it has been finished.

文档：工作机制
对异常数据的处理机制
