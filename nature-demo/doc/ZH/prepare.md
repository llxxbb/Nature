# Demo 项目运行准备

## 获取 Nature 可执行文件

可以通过下面的两种方式获取 Nature 的可执行文件。Nature 缺省使用 mysql 数据库，请自行准备，

### 下载已经编译好的版本

您可以直接[下载](https://github.com/llxxbb/Nature/releases)一个可执行的版本，暂时只发布win_64版，内含为Demo 服务的 executor：nature_demo.dll。

### 下载源代码并自行编译

下载项目代码: https://github.com/llxxbb/Nature

编译项目：然后进入 Nature 项目目录并运行下面的命令。 

```shell
cargo build
```

以 windows 环境进行说明，当编译完成后，在 target/debug目录下有下面的文件：

- nature.exe : Nature 的主程序.
- retry.exe : 为 Nature 重新加载因环境问题失败的任务，使其能够重新运行。
- nature_demo_restful.exe：示例项目，提供供 Nature 调用的 restful 接口
- nature_demo.dll:  ：示例项目，提供供 Nature 调用的本地库接口，将其复制到 .env 文件指定的 executor 目录下。

## 修改配置文件

Nature/.env 文件是项目的配置文件，将其拷贝到 target/debug目录下，并修改相应的值，下面为缺省的值。

```toml
DATABASE_URL=mysql://root@localhost/nature

REDO_URL=http://localhost:8080/task/redo

SERVER_PORT_NATURE=8080

EXECUTOR_PATH=executor
```

## 创建数据库

数据库的创建脚本位于 shell 目录下，并在 mysql 上执行 schema.sql

## 启动

进入target/debug目录，运行编译生成的三个可执行文件。