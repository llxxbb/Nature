//! World Connection Service provider
extern crate dotenv;
extern crate uuid;

use std::env;
use std::thread::JoinHandle;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

use crate::actor::*;
use crate::rpc::actix::*;
use nature_common::setup_logger;

// for product and mock
lazy_static! {
    pub static ref SERVER_PORT:String={
    env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string())
    };

    // sys biz define
    pub static ref SYS_KEY_SERIAL : String = "/serial".to_string();

    pub static ref PLAN_CONTENT_MAX_LENGTH : usize = {
        env::var("PLAN_CONTENT_MAX_LENGTH").unwrap_or_else(|_| "16777215".to_string()).parse::<usize>().unwrap()
    };

    pub static ref THREAD_NUM_FOR_STORE_ACTOR : usize = {
        env::var("THREAD_NUM_FOR_STORE_ACTOR").unwrap_or_else(|_| "3".to_string()).parse::<usize>().unwrap()
    };

    pub static ref THREAD_NUM_FOR_STORED_ACTOR : usize = {
        env::var("THREAD_NUM_FOR_STORED_ACTOR").unwrap_or_else(|_| "3".to_string()).parse::<usize>().unwrap()
    };

    pub static ref THREAD_NUM_FOR_CONVERT_ACTOR : usize = {
        env::var("THREAD_NUM_FOR_CONVERT_ACTOR").unwrap_or_else(|_| "3".to_string()).parse::<usize>().unwrap()
    };

    pub static ref QUERY_SIZE_LIMIT : usize = {
        env::var("QUERY_SIZE_LIMIT").unwrap_or_else(|_| "1000".to_string()).parse::<usize>().unwrap()
    };

}


pub fn sys_init() {
    dotenv().ok();
    let _ = setup_logger();
    start_actor();
}

fn start_actor() {
    init_actors();
    // web actor
    HttpServer::new(|| App::new().configure(web_config))
        .bind("127.0.0.1:".to_owned() + &SERVER_PORT)
        .unwrap()
        .run()
        .unwrap();
}

pub fn finish_threads<T>(threads: Vec<JoinHandle<T>>) {
    for t in threads {
        let _ = t.join();
    }
}


