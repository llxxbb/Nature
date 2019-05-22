#![feature(plugin)]
#![feature(trace_macros)]
#![feature(box_patterns)]
#![feature(proc_macro_hygiene, decl_macro)] // rocket

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
extern crate actix_web;
extern crate actix;

pub mod system;
pub mod rpc;
pub mod task;
mod channels;
