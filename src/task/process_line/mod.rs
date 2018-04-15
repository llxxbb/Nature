use dao::*;
use super::*;
use uuid::UuidBytes;

pub struct ProcessLine;

impl ProcessLine {
    /// born an instance which is the beginning of the changes.
    pub fn store(instance: Instance, root: Root) -> Result<UuidBytes> {
        let mut instance = instance;
        let uuid = InstanceImpl::verify(&mut instance, root)?;
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

    pub fn route(instance: Instance) {
        let _relations = MappingDaoService::get_relations(&instance);
    }
}

#[cfg(test)]
mod test_store;
