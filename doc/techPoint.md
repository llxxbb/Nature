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
+ uns lib.exe to convert dll to lib:

lib /def:sqlite3.def /out:sqlite3.lib

lib is under Path for Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.12.25827\bin\Hostx64\x64\lib.exe

+ set system environment variable

SQLITE3_LIB_DIR = sqlite3.lib path

+ attach dll path to `PATH` ENV， this is critical step

+ add config for cargo （if necessary）

### Use mysql

- download mysql connector for C version,  the C++ version connecter will be useless. address :https://cdn.mysql.com/archives/mysql-connector-c/mysql-connector-c-6.1.11-winx64.zip

  witch include a static lib file. in mysql-connector-c-6.1.11-winx64\lib\vs14

- unzip it and add system environment variable: MYSQLCLIENT_LIB_DIR

- I just finished it tonight, I think this is a easy way to use diesel_cli + mysql in windows. Please do the following three steps and you will get it.

  First: download a [mysql connecter](https://cdn.mysql.com/archives/mysql-connector-c/mysql-connector-c-6.1.11-winx64.zip)

  Second: unzip you had download and find the mysqlclient.lib in path mysql-connector-c-6.1.11-winx64\lib\vs14

  Third: set env variable MYSQLCLIENT_LIB_DIR to point which mysqlclient.lib lived in

  Run cargo install diesel_cli again

[question](https://stackoverflow.com/questions/54969208/how-to-link-mysql-client-installed-from-homebrew-with-diesel-cli)

I think mysql need not to be installed, cargo just need a lib, you can get it from mysql.

​    https://cdn.mysql.com/archives/mysql-connector-c/mysql-connector-c-6.1.11-winx64.zip

I found `mysqlclient.lib` in the following relative path.

​    \lib\vs14

In this way cargo install does not need `RUSTFLAGS` and succeed. 

Notice: you can't use c++ version connector,  there is no `mysqlclient.lib` in it.  And remember to set  environment variable MYSQLCLIENT_LIB_DIR

### install diesel_cli

cargo install diesel_cli --no-default-features --features sqlite,mysql



## markdown tools

[ghostwriter](https://github-production-release-asset-2e65be.s3.amazonaws.com/153566966/4ca08800-fc12-11e8-9dcd-05dbd0fd627d?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIWNJYAX4CSVEH53A%2F20190418%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20190418T092952Z&X-Amz-Expires=300&X-Amz-Signature=f19ce5ab65a917d911512697b68708c0f09e683326108fabe603bb34ccde1dbe&X-Amz-SignedHeaders=host&actor_id=22697285&response-content-disposition=attachment%3B%20filename%3Dghostwriter_x64_installer.exe&response-content-type=application%2Foctet-stream)

## Actix

all actors must be started between `System::new` and `System.run` . Otherwise the actor can't not handle the message.

version 0.7 problem :
use SyncArbiter can cause memory allocation error, it's queer.



