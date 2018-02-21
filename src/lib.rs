#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(proc_macro)]
extern crate fern;
#[macro_use]
extern crate log;
extern crate mock_derive;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


pub mod biz;
pub mod service;
pub mod rpc;
pub mod util;