extern crate nature;

use dotenv::dotenv;

use nature::rpc::actix::actix_start;

fn main() {
    dotenv().ok();
    actix_start();
}

//fn main() {
//    sys_init();
//    let _ = rocket_server().launch();
//}



