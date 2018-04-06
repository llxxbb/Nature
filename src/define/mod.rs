///! Common defines
extern crate diesel;

pub use self::error::*;
use std;


pub type Result<T> = std::result::Result<T, NatureError>;

pub mod error;



