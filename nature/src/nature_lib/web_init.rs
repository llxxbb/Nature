//! World Connection Service provider
extern crate dotenv;


use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use dotenv::dotenv;

use crate::nature_lib::web_controller::*;
use crate::util::{CHANNEL_BUNDLE_SIZE, SERVER_PORT, show_config};
use crate::util::channels::loop_receiver;
use crate::util::logger::logger_init;
use crate::util::web_context::WebContext;

pub async fn web_init() -> std::io::Result<()> {
    // need first execute
    dotenv().ok();
    logger_init();
    show_config();

    let (sx, rx) = async_channel::bounded(*CHANNEL_BUNDLE_SIZE);
    let context = Data::new(WebContext {
        sender: sx
    });

    let _ = loop_receiver(rx).await;
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
