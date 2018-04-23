use data::*;
use global::*;
#[cfg(test)]
pub use self::mock::*;
#[cfg(not(test))]
pub use self::process_line::*;
use serde::Serialize;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::thread;


#[derive(Debug, Serialize, Deserialize)]
pub struct RouteInfo {
    pub instance: Instance,
    pub maps: Vec<Mapping>,
}

#[derive(Debug)]
pub struct CarryError<T> where T: Sized + Serialize {
    pub err: NatureError,
    pub carrier: Carrier<T>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConverterTask(pub Instance, pub Mapping);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreTask(pub Instance);


pub mod process_line;

#[cfg(test)]
mod mock;