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

pub mod instance;
pub mod thing;
pub mod carrier;
pub mod mapping;
pub mod store_plan;

