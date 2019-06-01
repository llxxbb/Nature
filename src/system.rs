//! World Connection Service provider
extern crate dotenv;
extern crate uuid;

use std::env;
use std::thread::JoinHandle;

use actix::System;
use actix_web::server;

use nature_common::util::setup_logger;

use crate::channels::start_receive_threads;
use crate::rpc::actix::web_app;

use dotenv::dotenv;

// for product and mock
lazy_static! {
    pub static ref SERVER_PORT:String={
    env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string())
    };

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

    pub static ref THREAD_NUM_FOR_STORED_ACTOR : usize = {
        env::var("THREAD_NUM_FOR_STORED_ACTOR").unwrap_or_else(|_| "3".to_string()).parse::<usize>().unwrap()
    };

}


pub fn sys_init() {
    dotenv().ok();
    let _ = setup_logger();

    start_receive_threads();

    start_actor();
}

fn start_actor() {
    let sys = System::new("http-server");
    server::new(|| web_app())
        .bind("127.0.0.1:".to_owned() + &SERVER_PORT)
        .unwrap()
        .start();
    sys.run();
}

pub fn finish_threads<T>(threads: Vec<JoinHandle<T>>) {
    for t in threads {
        let _ = t.join();
    }
}


