use dao::*;
use data::*;
use db::*;
use super::*;

pub struct InstanceImpl;

impl InstanceImpl {
    fn id_generate_if_not_set(instance: &mut Instance) -> Result<UuidBytes> {
        let zero = instance.id.into_iter().all(|x| *x == 0);
        if zero {
            instance.id = generate_id(&instance.data)?;
        }
        Ok(instance.id)
    }


    /// check key whether defined
    /// generate id by hashing if it is not set.
    pub fn verify(instance: &mut Instance, root: Root) -> Result<UuidBytes> {
        Thing::key_standardize(&mut instance.data.thing.key, root)?;
        // just see whether it was configured.
        ThingDefineCache::get(&instance.data.thing)?;
        Self::id_generate_if_not_set(instance)
    }
}


#[cfg(test)]
mod test;