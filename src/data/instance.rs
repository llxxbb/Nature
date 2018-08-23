use global::*;
use super::*;
use util::*;


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

pub trait InstanceServiceTrait {
    fn verify(instance: &mut Instance) -> Result<u128>;
}

pub struct InstanceServiceImpl;

impl InstanceServiceTrait for InstanceServiceImpl {
    /// check key whether defined
    /// generate id by hashing if it is not set.
    fn verify(instance: &mut Instance) -> Result<u128> {
        Thing::key_standardize(&mut instance.data.thing.key)?;
        // just see whether it was configured.
        ThingDefineCacheImpl::get(&instance.data.thing)?;
        Self::id_generate_if_not_set(instance)
    }
}

impl InstanceServiceImpl {
    fn id_generate_if_not_set(instance: &mut Instance) -> Result<u128> {
        if instance.id == 0 {
            instance.id = generate_id(&instance.data)?;
        }
        Ok(instance.id)
    }
}
