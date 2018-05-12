///! rpc server, collect data from different rpc client then call the server

extern crate serde_json;

pub use self::rocket::*;

pub mod rocket;

