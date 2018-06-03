pub use self::carrier::*;
pub use self::instance::*;
pub use self::mapping::*;
pub use self::store_plan::*;
pub use self::thing::*;
#[cfg(test)]
pub use self::test::*;
use uuid::UuidBytes;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DelayedInstances {
    pub carrier_id: UuidBytes,
    pub result: CallbackResult,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CallbackResult {
    Err(String),
    Instances(Vec<Instance>),
}

impl Default for CallbackResult {
    fn default() -> Self {
        CallbackResult::Instances(Vec::new())
    }
}

mod instance;
mod thing;
mod carrier;
mod mapping;
mod store_plan;
#[cfg(test)]
mod test;

