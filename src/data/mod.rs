pub use self::carrier::*;
pub use self::instance::*;
pub use self::mapping::*;
pub use self::store_plan::*;
pub use self::thing::*;
use uuid::UuidBytes;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelayedInstances {
    pub carrier_id: UuidBytes,
    pub result: CallbackResult,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CallbackResult {
    Err(String),
    Instances(Vec<Instance>),
}

mod instance;
mod thing;
mod carrier;
mod mapping;
mod store_plan;

