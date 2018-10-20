# Technology Point

## Install a certain version of toolchain

example: rustup toolchain install nightly-2018-05-22

last: 1.29.0-nightly (e06c87544 2018-07-06)

## switch toolchain

restup default nightly-yyyy-mm-dd

## Environment setting.

## project edit under windows

A [reference](https://cmsd2.silvrback.com/rust-msvc) to use windows native dll for rust 

### make sqlite3.lib

+ Download sqlite for win664 and uppack
+ uns lib.exe to convert dll to lib:

lib /def:sqlite3.def /out:sqlite3.lib

lib is under Path for Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.12.25827\bin\Hostx64\x64\lib.exe

+ set system environment variable

SQLITE3_LIB_DIR = sqlite3.lib path

+ attach dll path to `PATH` ENV， this is critical step

+ add config for cargo （if necessary）

```toml
[target.x86_64-pc-windows-msvc.sqlite3]
rustc-link-lib = ["sqlite3"]
rustc-link-search = ["D:/common/sqlite3230100"]
root = "D:/common/sqlite3230100" 
```

### install diesel_cli

cargo install diesel_cli --no-default-features --features sqlite

## 架构设计理念

### 消除下一步操作中的“不确定的数据”
如，提前确定下一步要用的 `delivery.id`，这样在重新执行任务时就不会产生多余的副本。




