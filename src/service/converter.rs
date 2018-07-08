use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConverterInfo {
    pub from: Instance,
    pub target: Target,
    pub last_status: Option<Instance>,
}

pub struct CallOutParameter {
    pub from: Instance,
    pub last_status: Option<Instance>,
    /// This is used for callback
    pub carrier_id: u128,
}

pub enum ConverterReturned {
    Delay(u32),
    Instances(Vec<Instance>),
}
