extern crate nature;

use nature::rpc::*;
use nature::system::*;

fn main() {
    sys_init();
    let _ = start_rocket_server().launch();
}



