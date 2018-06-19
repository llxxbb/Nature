use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;
use super::*;


pub trait StoreTaskTrait {
    fn submit_single(instance: Instance) -> Result<u128>;
    fn store_with_root(carrier: Carrier<StoreInfo>, root: Root) -> Result<u128>;
    fn do_store(carrier: Carrier<StoreInfo>);
}

pub struct StoreTaskImpl<D, V, S, C, P> {
    delivery_trait: PhantomData<D>,
    instance_trait: PhantomData<V>,
    instance_dao: PhantomData<S>,
    thing_define_cache_trait: PhantomData<C>,
    dispatch_service: PhantomData<P>,
}

impl<D, V, S, C, P> StoreTaskTrait for StoreTaskImpl<D, V, S, C, P>
    where D: DeliveryTrait, V: InstanceTrait, S: InstanceDao, C: ThingDefineCacheTrait, P: DispatchTrait
{
    /// born an instance which is the beginning of the changes.
    fn submit_single(instance: Instance) -> Result<u128>
    {
        let task = StoreInfo { instance, converter: None };
        let biz = task.instance.data.thing.key.clone();
        let carrier = D::create_carrier(task, biz, DataType::Store as u8)?;
        Self::store_with_root(carrier, Root::Business)
    }

    fn store_with_root(carrier: Carrier<StoreInfo>, root: Root) -> Result<u128> {
        let mut instance = carrier.content.data.instance.clone();
        let uuid = V::verify(&mut instance, root)?;
        let result = S::insert(&instance);
        match result {
            Ok(_) => {
                send_carrier(CHANNEL_ROUTE.sender.lock().unwrap().clone(), carrier);
                Ok(uuid)
            }
            Err(NatureError::DaoDuplicated) => {
                Self::handle_duplicated(carrier, instance)?;
                Ok(uuid)
            }
            Err(err) => Err(err)
        }
    }

    fn do_store(carrier: Carrier<StoreInfo>) {
        if let Err(err) = Self::store_with_root(carrier.clone(), Root::Business) {
            DeliveryImpl::<TableDelivery>::move_to_err(err, carrier)
        };
    }
}

impl<D, V, S, C, P> StoreTaskImpl<D, V, S, C, P>
    where D: DeliveryTrait, C: ThingDefineCacheTrait, P: DispatchTrait
{
    fn handle_duplicated(carrier: Carrier<StoreInfo>, instance: Instance) -> Result<()> {
        let define = C::get(&instance.data.thing)?;
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