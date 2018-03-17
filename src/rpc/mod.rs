///! rpc server, collect data from different rpc client then call the server

extern crate serde_json;

pub use self::rocket::start_rocket_server;

pub mod rocket;

