use data::*;
use db::*;
use global::*;
pub use self::channel::*;
#[cfg(test)]
pub use self::mock::*;
#[cfg(not(test))]
pub use self::process_line::*;
pub use self::structure_impl::*;
use serde::Serialize;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::thread;
use uuid::UuidBytes;

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
    pub last_status: Option<Instance>,
}

pub struct CallOutParameter {
    pub from: Instance,
    pub last_status: Option<Instance>,
    /// This is used for callback
    pub carrier_id: UuidBytes,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreInfo {
    pub instance: Instance,
    /// save outside has non converter info.
    pub converter: Option<ConverterInfo>,
}

pub enum ConverterReturned {
    Delay(u32),
    Instances(Vec<Instance>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialFinished {
    succeeded_id: Vec<UuidBytes>,
    errors: Vec<String>,
}

mod process_line;

mod structure_impl;

mod channel;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;


