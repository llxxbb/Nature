# 项目准备

## 获取可执行文件

您可以直接[下载](https://github.com/llxxbb/Nature/releases)一个可执行的版本，暂时只发布win_64版，且不包含Demo相关的组件。

Nature 缺省使用  mysql 数据库，请自行准备，下面是自行编译的方法，以 windows 环境进行说明：


### 下载代码

下载项目代码: https://github.com/llxxbb/Nature

### 编译项目

然后进入 Nature 子目录并运行下面的命令。 

```shell
cargo build
```

当编译完成后，在 Nature/target目录下有三个可执行文件：

- nature.exe : Nature 的主程序.
- retry.exe : 为 Nature 重新加载因环境问题失败的任务，使其能够重新运行。
- restful_executor.exe：服务于示例项目的基于restful的转换器实现


## 修改配置文件

Nature/.env 文件是项目的配置文件，将其拷贝到Nature/target目录下，并修改相应的值，下面为缺省的值。

```toml
DATABASE_URL=mysql://root@localhost/nature

NATURE_SERVER_ADDRESS=http://localhost:8080/task/redo

SERVER_PORT_NATURE=8080
```
## 创建数据库

数据库的创建脚本位于Nature-DB/migrations目录下。如果你安装了diesel_cli，你可以在终端上运行下面的命令便可完成数据库的初始化：

```shell
diesel migration run
```

## 启动

进入Nature/target目录，运行编译生成的三个可执行文件。