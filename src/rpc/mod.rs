///! rpc server, collect data from different rpc client then call the server

extern crate serde_json;

pub mod web;
pub use self::web::start_web_server;


