extern crate dotenv;
///! World Connection Service provider
extern crate uuid;

use flow::*;
use nature_common::util::setup_logger;
use self::dotenv::dotenv;
use std::thread::JoinHandle;

// for product and mock
lazy_static! {
    // sys biz define
    pub static ref SYS_KEY_SERIAL : String = "/serial".to_string();

    // sys context define
    pub static ref CONTEXT_TARGET_INSTANCE_ID : String = "sys.target_instance_id".to_string();
}

pub fn sys_init() -> Vec<JoinHandle<()>> {
    dotenv().ok();
    let _ = setup_logger();

    Controller::start()
}

pub fn finish_threads<T>(threads: Vec<JoinHandle<T>>) {
    for t in threads {
        let _ = t.join();
    }
}


