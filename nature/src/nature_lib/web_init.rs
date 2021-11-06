//! World Connection Service provider
extern crate dotenv;

use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;

use crate::nature_lib::web_controller::*;
use crate::util::channels::start_receive_threads;
use crate::util::show_config;

lazy_static! {
    pub static ref SERVER_PORT:String={
    env::var("SERVER_PORT_NATURE").unwrap_or_else(|_| "8080".to_string())
    };
}

pub async fn web_init() -> std::io::Result<()> {
    dotenv().ok();
    let _ = env_logger::init();
    show_config();
    let _ = start_receive_threads();
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .configure(web_config)
    })
        .bind("0.0.0.0:".to_owned() + &SERVER_PORT).unwrap()
        .run().await
}
