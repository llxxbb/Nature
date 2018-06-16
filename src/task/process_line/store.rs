use std::ops::Deref;
use std::sync::Arc;
use super::*;


pub trait StoreTaskTrait {
    fn submit_single<D, V, S, C>(_: &D, verify_instance: &V, store_instance: &S, thing_define: &C, instance: Instance) -> Result<u128>
        where D: DeliveryTrait, V: InstanceTrait, S: InstanceDao, C: ThingDefineCacheTrait;
    fn store_with_root<V, S, C>(verify_instance: &V, store_instance: &S, thing_define: &C, carrier: Carrier<StoreInfo>, root: Root) -> Result<u128>
        where V: InstanceTrait, S: InstanceDao, C: ThingDefineCacheTrait;
    fn do_store(carrier: Carrier<StoreInfo>);
}

pub struct StoreTaskImpl;

impl StoreTaskTrait for StoreTaskImpl {
    /// born an instance which is the beginning of the changes.
    fn submit_single<D, V, S, C>(_: &D, verify_instance: &V, store_instance: &S, thing_define: &C, instance: Instance) -> Result<u128>
        where D: DeliveryTrait, V: InstanceTrait, S: InstanceDao, C: ThingDefineCacheTrait {
        let task = StoreInfo { instance, converter: None };
        let carrier = D::create_carrier(task)?;
        Self::store_with_root(verify_instance, store_instance, thing_define, carrier, Root::Business)
    }

    fn store_with_root<V, S, C>(_verify_instance: &V, _store_instance: &S, _thing_define: &C, carrier: Carrier<StoreInfo>, root: Root) -> Result<u128>
        where V: InstanceTrait, S: InstanceDao, C: ThingDefineCacheTrait {
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

    fn do_store(carrier: Carrier<StoreInfo>) {
        if let Err(err) = StoreTaskImpl::store_with_root(DATA_INSTANCE.clone().deref(),
                                                         DAO_INSTANCE.clone().deref(),
                                                         CACHE_THING_DEFINE.clone().deref(),
                                                         carrier.clone(),
                                                         Root::Business) {
            DeliveryImpl::move_to_err(err, carrier)
        };
    }
}

fn handle_duplicated(carrier: Carrier<StoreInfo>, instance: Instance) -> Result<()> {
    let define = ThingDefineCacheImpl::get(&instance.data.thing)?;
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