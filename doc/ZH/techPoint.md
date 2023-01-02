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
