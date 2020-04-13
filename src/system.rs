//! World Connection Service provider
extern crate dotenv;
extern crate uuid;

use std::env;
use std::thread::JoinHandle;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

use nature_common::setup_logger;

use crate::rpc::actix::*;

lazy_static! {
    pub static ref SERVER_PORT:String={
    env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string())
    };

    pub static ref SWITCH_SAVE_DIRECTLY_FOR_ONE : bool = {
        env::var("SWITCH_SAVE_DIRECTLY_FOR_ONE").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap()
    };
}

pub async fn sys_init() -> std::io::Result<()> {
    dotenv().ok();
    let _ = setup_logger();
    HttpServer::new(|| App::new().configure(web_config))
        .bind("127.0.0.1:".to_owned() + &SERVER_PORT).unwrap()
        .run().await
}

pub fn finish_threads<T>(threads: Vec<JoinHandle<T>>) {
    for t in threads {
        let _ = t.join();
    }
}


