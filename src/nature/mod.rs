use define::*;
use instance::*;
use std::thread;
use uuid::*;
use carrier::*;
use task::*;
use service::*;

pub trait Nature: Sync + Send {
    fn flow(&self, thing: Instance) -> Result<UuidBytes>;
}

pub struct NatureService;

impl Nature for NatureService {
    fn flow(&self, instance: Instance) -> Result<UuidBytes> {
        let mut instance = instance;
        let uuid = instance.verify(&*DEFINE_DAO)?;
        let task = StoreTask(instance);
        let carrier = Carrier::new(task)?;
        carrier.take_it_over()?;
        let sender = PROCESSOR_ROUTE.sender.lock().unwrap().clone();
        thread::spawn(move || {
            sender.send(carrier).unwrap();
        });
        Ok(uuid)
    }
}

#[cfg(test)]
mod test;

