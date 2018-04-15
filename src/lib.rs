#![feature(plugin, proc_macro)]
#![plugin(rocket_codegen)]
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
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub mod global;
pub mod rpc;
pub mod util;
pub mod convert;
pub mod task;
pub mod data;
