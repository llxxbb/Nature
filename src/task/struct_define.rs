use data::*;
use global::*;
use serde::Serialize;

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
    pub carrier_id: u128,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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
    pub succeeded_id: Vec<u128>,
    pub errors: Vec<String>,
}
