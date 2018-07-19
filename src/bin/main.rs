extern crate chrono;
extern crate fern;
extern crate nature;

use nature::global::*;
use nature::rpc::*;

fn main() {
    sys_init();
    let _ = start_rocket_server().launch();
}



