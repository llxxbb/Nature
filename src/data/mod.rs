//! Define the data used all over the project, not only by `fg-service`

use nature_common::*;
pub use self::cache::*;
pub use self::converter_cfg::*;
pub use self::delivery::*;
pub use self::instance::{InstanceServiceImpl, InstanceServiceTrait};
pub use self::orm::*;
pub use self::sqlite::*;
#[cfg(test)]
pub use self::test::*;
pub use self::thing::*;
pub use self::trait_define::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DelayedInstances {
    pub carrier_id: u128,
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

/// **unique key**
/// * from_id
/// * from_thing
#[derive(Debug)]
pub struct PlanInfo {
    pub from_id: u128,
    pub from_thing: Thing,
    pub to: Thing,
    pub plan: Vec<Instance>,
}


mod thing;
mod delivery;
#[cfg(test)]
mod test;
mod converter_cfg;

mod sqlite;
mod cache;
mod trait_define;
mod orm;
mod instance;