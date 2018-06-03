use std::ops::Deref;
use std::sync::Arc;
use super::*;


pub trait StoreTaskTrait {
    fn submit_single<D, V, S>(delivery: &D, verify_instance: &V, store_instance: &S, instance: Instance) -> Result<UuidBytes>
        where D: DeliveryTrait, V: InstanceTrait, S: InstanceDao;
    fn do_store<V, S>(verify_instance: &V, store_instance: &S, carrier: Carrier<StoreInfo>, root: Root) -> Result<UuidBytes>
        where V: InstanceTrait, S: InstanceDao;
    fn receive(carrier: Carrier<StoreInfo>);
}

pub struct StoreTaskImpl;

impl StoreTaskTrait for StoreTaskImpl {
    /// born an instance which is the beginning of the changes.
    fn submit_single<D, V, S>(_: &D, verify_instance: &V, store_instance: &S, instance: Instance) -> Result<UuidBytes>
        where D: DeliveryTrait, V: InstanceTrait, S: InstanceDao {
        let task = StoreInfo { instance, converter: None };
        let carrier = D::create_carrier(task)?;
        Self::do_store(verify_instance, store_instance, carrier, Root::Business)
    }

    fn do_store<V, S>(_: &V, _: &S, carrier: Carrier<StoreInfo>, root: Root) -> Result<UuidBytes>
        where V: InstanceTrait, S: InstanceDao {
        let mut instance = carrier.data.instance.clone();
        let uuid = V::verify(&mut instance, root)?;
        let result = S::insert(&instance);
        match result {
            Ok(_) => {
                send_carrier(CHANNEL_ROUTE.sender.lock().unwrap().clone(), carrier);
                Ok(uuid)
            }
            Err(NatureError::DaoDuplicated) => {
                handle_duplicated(carrier, instance)?;
                Ok(uuid)
            }
            Err(err) => Err(err)
        }
    }

    fn receive(carrier: Carrier<StoreInfo>) {
        if let Err(err) = StoreTaskImpl::do_store(
            DATA_INSTANCE.clone().deref(),
            DAO_INSTANCE.clone().deref(),
            carrier.clone(),
            Root::Business) {
            DeliveryImpl::move_to_err(err, carrier)
        };
    }
}

fn handle_duplicated(carrier: Carrier<StoreInfo>, instance: Instance) -> Result<()> {
    let define = ThingDefineCache::get(&instance.data.thing)?;
    if define.is_status() {
        // status need to retry and correct the status version.
        re_dispatch(carrier)
    } else {
        // **None Status Thing** won't try again
        DeliveryImpl::finish_carrier(&carrier.id)?;
        Ok(())
    }
}

lazy_static! {
    pub static ref TASK_STORE : Arc<StoreTaskImpl> = Arc::new(StoreTaskImpl);
}