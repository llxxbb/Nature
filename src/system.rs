//! World Connection Service provider
extern crate dotenv;
extern crate uuid;

use channels::start_receive_threads;
use flow::*;
use nature_common::util::setup_logger;
use self::dotenv::dotenv;
use std::thread::JoinHandle;

// for product and mock
lazy_static! {
    // sys biz define
    pub static ref SYS_KEY_SERIAL : String = "/serial".to_string();

    // sys context define
    pub static ref CONTEXT_TARGET_INSTANCE_ID : String = "sys.target".to_string();

    // controller
    pub static ref SVC_NATURE : ControllerImpl = ControllerImpl::new();
}

pub fn sys_init() -> Vec<JoinHandle<()>> {
    dotenv().ok();
    let _ = setup_logger();

    start_receive_threads()
}

pub fn finish_threads<T>(threads: Vec<JoinHandle<T>>) {
    for t in threads {
        let _ = t.join();
    }
}


