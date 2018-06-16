pub use self::carrier::*;
pub use self::instance::*;
pub use self::relation::*;
pub use self::store_plan::*;
#[cfg(test)]
pub use self::test::*;
pub use self::thing::*;

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

mod instance;
mod thing;
mod carrier;
mod relation;
mod store_plan;
#[cfg(test)]
mod test;

