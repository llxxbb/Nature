#![feature(plugin)]
#![feature(trace_macros)]
#![feature(box_patterns)]

extern crate actix;
extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate lru_time_cache;
extern crate nature_common;
extern crate nature_db;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub mod system;
pub mod rpc;
pub mod task;
pub mod channels;
pub mod actor;
