//! World Connection Service provider
extern crate dotenv;

use std::env;
use std::sync::Arc;
use std::thread::JoinHandle;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

use nature_common::setup_logger;
use nature_db::{InstanceDaoImpl, KeyRange};

use crate::channels::start_receive_threads;
use crate::web::actix::*;

// use actix_web::middleware::Logger;

lazy_static! {
    pub static ref INS_KEY_GT : Arc<dyn KeyRange> = Arc::new(InstanceDaoImpl{});
}

lazy_static! {
    pub static ref SERVER_PORT:String={
    env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string())
    };

    pub static ref SWITCH_SAVE_DIRECTLY_FOR_ONE : bool = {
        env::var("SWITCH_SAVE_DIRECTLY_FOR_ONE").unwrap_or_else(|_| "true".to_string()).parse::<bool>().unwrap()
    };

    pub static ref CACHE_SAVED_TIME : u64 = {
        env::var("CACHE_SAVED_TIME").unwrap_or_else(|_| "90".to_string()).parse::<u64>().unwrap()
    };
}

pub async fn sys_init() -> std::io::Result<()> {
    dotenv().ok();
    // std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = setup_logger();
    let _ = start_receive_threads();
    HttpServer::new(|| App::new()
        // .wrap(Logger::default())
        .configure(web_config))
        .bind("127.0.0.1:".to_owned() + &SERVER_PORT).unwrap()
        .run().await
}

pub fn finish_threads<T>(threads: Vec<JoinHandle<T>>) {
    for t in threads {
        let _ = t.join();
    }
}


