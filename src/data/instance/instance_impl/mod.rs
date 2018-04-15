use dao::*;
use data::carrier::*;
use std::thread;
use super::*;
use task::*;

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
        ThingDefineDaoService::get(&instance.data.thing)?;
        Self::id_generate_if_not_set(instance)
    }
}

impl InstanceTrait for InstanceImpl {
    fn born(instance: Instance) -> Result<UuidBytes> {
        let mut instance = instance;
        let uuid = InstanceImpl::verify(&mut instance, Root::Business)?;
        let task = StoreTask(instance);
        let carrier = Carrier { data: task };
        let _cid = CarrierDaoService::insert(&carrier)?;
        carrier.take_it_over()?;
        let sender = CHANNEL_ROUTE.sender.lock().unwrap().clone();
        thread::spawn(move || {
            sender.send(carrier.data.0).unwrap();
        });
        Ok(uuid)
    }
    fn serial(_batch: SerialBatchInstance) -> Result<()> {
        // TODO
        unimplemented!()
    }

    fn parallel(_batch: ParallelBatchInstance) -> Result<()> {
        // TODO
        unimplemented!()
    }
}


#[cfg(test)]
mod test;