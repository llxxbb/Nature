# Technology Point

## Install a certain version of toolchain

rustup toolchain install nightly-2018-12-08

## switch toolchain

restup default nightly-yyyy-mm-dd

## Environment setting.

## clippy

rustup component add clippy-preview

## project edit under windows

A [reference](https://cmsd2.silvrback.com/rust-msvc) to use windows native dll for rust 

### make sqlite3.lib

+ Download sqlite-dll for win64 and unpack it (.def and .dll will be used for the cmd lib)
+ use lib.exe to convert dll to lib:

lib /def:sqlite3.def /out:sqlite3.lib

lib is under Path for Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.12.25827\bin\Hostx64\x64\lib.exe

+ set system environment variable

SQLITE3_LIB_DIR = sqlite3.lib path

+ attach dll path to `PATH` ENV， this is critical step

+ add config for cargo （if necessary）

### lock-free for channel

The self-contained channel + Mutex combination is a lock structure, but crossbeam is a lock-free data structure.

这里使用 smol 的 async_channel 库

[Rust 并发编程 - Thread Pool - 简书 (jianshu.com)](https://www.jianshu.com/p/f4d853c0ef1e)
[【Rust每周一库】smol - 异步rumtime - Rust语言中文社区 (rustcc.cn)](https://rustcc.cn/article?id=2a02d42f-4b27-40f1-ad0e-2015d3413bb7)
