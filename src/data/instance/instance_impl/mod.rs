use data::carrier::*;
use data::dao::*;
use std::thread;
use super::*;
use task::*;
use util::*;


impl Instance {
    pub fn new_batch_for_serial(batch: &mut SerialBatchInstance) -> Result<Instance> {
        // veriry all
        for mut instance in &mut batch.instance {
            InstanceImpl::verify(&mut instance)?;
        }
        let instance = Instance {
            id: {
                // id based on instance list in `SerialBatchInstance`
                let vec = batch.instance.iter().map(|x| &x.id).collect::<Vec<_>>();
                generate_id(&vec)?
            },
            data: InstanceNoID {
                thing: Thing {
                    key: SYS_KEY_BATCH_SERIAL.to_string(),
                    version: 1,
                },
                execute_time: Local::now().timestamp_millis(),
                create_time: Local::now().timestamp_millis(),
                content: String::new(),
                context: String::new(),
            },
        };
        Ok(instance)
    }
}

pub struct InstanceImpl;

impl InstanceImpl {
    fn id_generate_if_not_set(instance: &mut Instance) -> Result<UuidBytes> {
        let zero = instance.id.into_iter().all(|x| *x == 0);
        if zero {
            instance.id = generate_id(&instance.data)?;
        }
        Ok(instance.id)
    }

    pub fn verify(instance: &mut Instance) -> Result<UuidBytes> {
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