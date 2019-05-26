//! World Connection Service provider
extern crate dotenv;
extern crate uuid;

use std::env;
use std::thread::JoinHandle;

use nature_common::util::setup_logger;

use crate::channels::start_receive_threads;

use self::dotenv::dotenv;

// for product and mock
lazy_static! {
    // sys biz define
    pub static ref SYS_KEY_SERIAL : String = "/serial".to_string();

    // sys context define
    pub static ref CONTEXT_TARGET_INSTANCE_ID : String = "sys.target".to_string();

    pub static ref PLAN_CONTENT_MAX_LENGTH : usize = {
        env::var("PLAN_CONTENT_MAX_LENGTH").unwrap_or_else(|_| "16777215".to_string()).parse::<usize>().unwrap()
    };

    pub static ref THREAD_NUM_FOR_STORE_ACTOR : usize = {
        env::var("THREAD_NUM_FOR_STORE_ACTOR").unwrap_or_else(|_| "3".to_string()).parse::<usize>().unwrap()
    };
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


