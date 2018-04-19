use dao::*;
use self::dispatch_task::*;
use super::*;
use uuid::UuidBytes;

pub struct ProcessLine;

impl ProcessLine {
    /// born an instance which is the beginning of the changes.
    pub fn store(instance: Instance, root: Root) -> Result<UuidBytes> {
        let mut instance = instance;
        let uuid = InstanceImpl::verify(&mut instance, root)?;
        let task = StoreTask(instance);
        let carrier = Carrier::new(task)?;
        let _cid = CarrierDaoService::insert(&carrier)?;
        carrier.take_it_over()?;
        let sender = CHANNEL_ROUTE.sender.lock().unwrap().clone();
        thread::spawn(move || {
            sender.send(carrier).unwrap();
        });
        Ok(uuid)
    }

    pub fn route(carrier: Carrier<StoreTask>) {
        if let Ok(x) = MappingDaoService::get_relations(&carrier.data.0) {
            if let Some(y) = x {
                let task = DispatchTask(y);
                if let Ok(new_carrier) = Carrier::new(task) {
                    if let Ok(_) = CarrierDaoService::insert(&new_carrier) {
                        let _unused = new_carrier.take_it_over();
                    };
                };
            };
        };
    }
}

#[cfg(test)]
mod test_store;
