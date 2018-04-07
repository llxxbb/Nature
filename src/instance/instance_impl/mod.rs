use carrier::*;
use dao::*;
use instance::instance_trait::InstanceTrait;
use serde_json;
use service::*;
use std::thread;
use super::*;
use task::*;

pub struct InstanceImpl;

impl InstanceImpl {
    fn id_generate_if_not_set(instance: &mut Instance) -> Result<UuidBytes> {
        let zero = instance.id.into_iter().all(|x| *x == 0);
        if zero {
            let json = serde_json::to_string(&instance.data)?;
            instance.id = *Uuid::new_v3(&NAMESPACE_DNS, &json).as_bytes();
        }
        Ok(instance.id)
    }

    fn verify(instance: &mut Instance) -> Result<UuidBytes> {
        // just see whether it was configured.
        let mut dao = DEFINE_DAO.lock().unwrap();
        let _def = dao.get(&instance.data.thing)?;
        Self::id_generate_if_not_set(instance)
    }
}

impl InstanceTrait for InstanceImpl {
    fn born(instance: Instance) -> Result<UuidBytes> {
        let mut instance = instance;
        let uuid = InstanceImpl::verify(&mut instance)?;
        let task = StoreTask(instance);
        let carrier = Carrier { data: task };
        let _cid = CarrierDaoService::insert(&carrier)?;
        carrier.take_it_over()?;
        let sender = PROCESSOR_ROUTE.sender.lock().unwrap().clone();
        thread::spawn(move || {
            sender.send(carrier).unwrap();
        });
        Ok(uuid)
    }


    fn store(_instance: Instance) -> Result<()> {
        // TODO
        unimplemented!();
    }
}


#[cfg(test)]
mod test;