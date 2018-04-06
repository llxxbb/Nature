use define::*;
use instance::*;
use std::thread;
use super::*;
use uuid::*;

pub trait Nature: Sync + Send {
    fn flow(&self, thing: Instance) -> Result<UuidBytes>;
}

pub struct NatureService;

impl Nature for NatureService {
    fn flow(&self, instance: Instance) -> Result<UuidBytes> {
        let mut instance = instance;
        let uuid = instance.verify()?;
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

