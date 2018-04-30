use data::*;
use global::*;
pub use self::call_out::*;
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
pub struct ConverterInfo {
    pub from: Instance,
    pub mapping: Mapping,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreInfo {
    pub instance: Instance,
    /// save outside has non converter info.
    pub converter: Option<ConverterInfo>,
}

pub struct ParaForCallOut {
    pub from: Instance,
    pub last_target: Option<Instance>,
    pub for_callback: Vec<u8>,
}

pub mod process_line;

mod call_out;

#[cfg(test)]
mod mock;

