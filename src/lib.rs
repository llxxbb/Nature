#![feature(plugin, proc_macro, proc_macro_gen)]
#![plugin(rocket_codegen)]
#![feature(range_contains)]
#![feature(extern_prelude)]
#![feature(trace_macros)]
#![feature(int_to_from_bytes)]  // this used to convert uuid to u128
#![feature(try_from)] // vec to array

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
extern crate rand;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub mod global;
pub mod util;
pub mod rpc;
pub mod data;
pub mod db;
pub mod service;

