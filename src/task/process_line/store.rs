use std::marker::PhantomData;
use std::sync::Arc;
use super::*;


pub trait StoreTrait {
    fn input(instance: Instance) -> Result<u128>;
    fn do_store_task(carrier: Carrier<StoreInfo>);
}

pub struct StoreTaskImpl<D, V, S, C, P> {
    delivery_trait: PhantomData<D>,
    instance_trait: PhantomData<V>,
    instance_dao: PhantomData<S>,
    thing_define_cache_trait: PhantomData<C>,
    dispatch_service: PhantomData<P>,
}

impl<D, V, S, C, P> StoreTrait for StoreTaskImpl<D, V, S, C, P>
    where D: DeliveryTrait, V: InstanceTrait, S: InstanceDao, C: ThingDefineCacheTrait, P: DispatchTrait
{
    /// born an instance which is the beginning of the changes.
    fn input(mut instance: Instance) -> Result<u128>
    {
        // verify input
        let uuid = V::verify(&mut instance, Root::Business)?;
        // save to delivery to make it can redo
        let task = StoreInfo { instance, converter: None };
        let biz = task.instance.data.thing.key.clone();
        let carrier = D::create_carrier(task, biz, DataType::Store as u8)?;
        // to delivery
        send_carrier(CHANNEL_STORE.sender.lock().unwrap().clone(), carrier);
        Ok(uuid)
    }

    fn do_store_task(carrier: Carrier<StoreInfo>) {
        if let Err(err) = Self::store_with_root(carrier.clone()) {
            DeliveryImpl::<TableDelivery>::move_to_err(err, carrier)
        };
    }
}

impl<D, V, S, C, P> StoreTaskImpl<D, V, S, C, P>
    where D: DeliveryTrait, C: ThingDefineCacheTrait, P: DispatchTrait, S: InstanceDao
{
    fn store_with_root(carrier: Carrier<StoreInfo>) -> Result<u128> {
        let id = carrier.instance.id;
        let result = S::insert(&carrier.instance);
        match result {
            Ok(_) => {
                send_carrier(CHANNEL_ROUTE.sender.lock().unwrap().clone(), carrier);
                Ok(id)
            }
            Err(NatureError::DaoDuplicated) => {
                Self::handle_duplicated(carrier)?;
                Ok(id)
            }
            Err(err) => Err(err)
        }
    }

    fn handle_duplicated(carrier: Carrier<StoreInfo>) -> Result<()> {
        let define = C::get(&carrier.instance.data.thing)?;
        if define.is_status() {
            // status need to retry and correct the status version.
            P::re_dispatch(carrier)
        } else {
            // **None Status Thing** won't try again
            D::finish_carrier(&carrier.id)?;
            Ok(())
        }
    }
}

pub type StoreTask = StoreTaskImpl<DeliveryImpl<TableDelivery>, InstanceImpl, TableInstance, ThingDefineCacheImpl, DispatchTask>;

lazy_static! {
    pub static ref TASK_STORE : Arc<StoreTask> = Arc::new(StoreTaskImpl{
        delivery_trait: PhantomData,
    instance_trait: PhantomData,
    instance_dao: PhantomData,
    thing_define_cache_trait: PhantomData,
    dispatch_service: PhantomData,
    });
}