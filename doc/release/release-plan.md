# 计划发布的版本

发布原则：Demo 项目，单元测试都通过 计划的总体原则：

- 功能性先于非功能性需求
- 应用情景支持，内部优先于外部（如网关）

下面为正在开发的内容

Release 1.5.0

- 修正 nature-demo/doc/ZH/prepare.md 中的问题.
- 将 NatureError 和 Result 移动到 common 模块下。
- release 中包含 demo 所需的执行器和可执行文件
- 修订 Readme 文件
- 增加本地 executor 位置的设定。

### It should commit


### 未完成

remove support for sqlite

为META 和 relation 增加 id 字段。

- 测试异常的任务

optimize: cancel run the same task when it has been finished.

文档：工作机制
对异常数据的处理机制
