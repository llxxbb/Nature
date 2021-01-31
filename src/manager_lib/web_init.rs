use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;

use crate::manager_lib::web_controller::manager_config;

lazy_static! {
    static ref SERVER_PORT:String={
        env::var("SERVER_PORT_MANAGER").unwrap_or_else(|_| "8180".to_string())
    };
    static ref CLIENT_URL:String={
        let rtn = env::var("MANAGER_CLIENT_URL").unwrap_or_else(|_| "http://localhost:8280".to_string());
        info!("MANAGER_CLIENT_URL: {:?}", rtn);
        rtn
    };
}

pub async fn web_init() -> std::io::Result<()> {
    dotenv().ok();
    let _ = env_logger::init();
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin(&CLIENT_URL)
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .configure(manager_config)
    })
        .bind("127.0.0.1:".to_owned() + &SERVER_PORT).unwrap()
        .run().await
}
