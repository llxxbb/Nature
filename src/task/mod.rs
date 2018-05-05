use data::*;
use global::*;
pub use self::converter_info_impl::*;
pub use self::process_line::*;
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

impl CallOutParameter {
    pub fn new(internal: &Carrier<ConverterInfo>) -> Self {
        CallOutParameter {
            from: internal.from.clone(),
            last_status: internal.last_status.clone(),
            carrier_id: internal.id.clone(),
        }
    }
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


mod converter_info_impl;


