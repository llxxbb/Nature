# project edit under windows

A [reference](https://cmsd2.silvrback.com/rust-msvc) to use windows native dll for rust 

## make sqlite3.lib

+ Download sqlite for win664 and uppack
+ uns lib.exe to convert dll to lib:

lib /def:sqlite3.def /out:sqlite3.lib

lib is under Path for Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.12.25827\bin\Hostx64\x64\lib.exe

+ set system environment variable

SQLITE3_LIB_DIR = sqlite3.lib path

+ attach dll path to `PATH` ENV， this is critical step

+ add config for cargo

```toml
[target.x86_64-pc-windows-msvc.sqlite3]
rustc-link-lib = ["sqlite3"]
rustc-link-search = ["D:/common/sqlite3230100"]
root = "D:/common/sqlite3230100" 
```

## install diesel_cli

cargo install diesel_cli --no-default-features --features sqlite

## addition

Maybe you need the following things

### Chocolatey

A package manager for windows: [install](https://chocolatey.org/install#install-with-powershellexe)