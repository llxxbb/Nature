//! World Connection Service provider
extern crate dotenv;

use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use dotenv::dotenv;

use crate::nature_lib::web_controller::*;
use crate::util::channels::start_receive_threads;
use crate::util::show_config;
use crate::util::web_context::WebContext;

lazy_static! {
    pub static ref SERVER_PORT:String={
        env::var("SERVER_PORT_NATURE").unwrap_or_else(|_| "8080".to_string())
    };
    pub static ref CHANNEL_SIZE:usize={
        env::var("CHANNEL_SIZE").unwrap_or_else(|_| "1000".to_string()).parse().unwrap()
    };
}

pub async fn web_init() -> std::io::Result<()> {
    let (sx, rx) = async_channel::bounded(*CHANNEL_SIZE);

    let context = Data::new(WebContext {
        sender: sx
    });


    dotenv().ok();
    let _ = env_logger::init();
    show_config();
    let _ = start_receive_threads(rx);
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(context.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .configure(web_config)
    })
        .bind("0.0.0.0:".to_owned() + &SERVER_PORT).unwrap()
        .run().await
}
