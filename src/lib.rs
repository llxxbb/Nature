#![feature(plugin, proc_macro)]
#![plugin(rocket_codegen)]
#![plugin(mockers_macros)]
extern crate fern;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate mockers;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


pub mod biz;
pub mod service;
pub mod rpc;
pub mod util;