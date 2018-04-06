#![feature(plugin, proc_macro)]
#![plugin(rocket_codegen)]
//#![plugin(mockers_macros)]
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate fern;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate lru_time_cache;
extern crate r2d2;
extern crate r2d2_diesel;
//#[cfg(test)]
//extern crate mockers;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;


pub mod define;
pub mod service;
pub mod rpc;
pub mod util;
pub mod carrier;
pub mod convert;
pub mod dao;
pub mod instance;
pub mod stream;
pub mod thing;
pub mod task;
pub mod processor;
pub mod nature;
