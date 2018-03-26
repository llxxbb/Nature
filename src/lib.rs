#![feature(plugin, proc_macro)]
#![plugin(rocket_codegen)]
//#![plugin(mockers_macros)]
extern crate fern;
#[macro_use]
#[cfg(test)]
extern crate lazy_static;
#[macro_use]
extern crate log;
//#[cfg(test)]
//extern crate mockers;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;
extern crate chrono;
extern crate lru_time_cache;


pub mod define;
pub mod service;
pub mod rpc;
pub mod util;
pub mod transmit;
pub mod convert;
pub mod dao;
pub mod store;