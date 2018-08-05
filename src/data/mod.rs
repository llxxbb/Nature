//! Define the data used all over the project, not only by `fg-service`

pub use self::converter::*;
pub use self::delivery::*;
pub use self::instance::*;
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
mod converter;
mod delivery;
#[cfg(test)]
mod test;

