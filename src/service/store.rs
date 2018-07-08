use std::marker::PhantomData;
use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StoreTaskInfo {
    pub instance: Instance,
    /// save outside has non converter info.
    pub upstream: Option<ConverterInfo>,
    pub target: Option<Vec<Target>>,
}

pub trait StoreServiceTrait {
    /// verify input data first then `do_store`
    fn input(instance: Instance) -> Result<u128>;
    fn receive_store_task(carrier: Carrier<StoreTaskInfo>);
    fn send_store_task(task: StoreTaskInfo) -> Result<()>;
    fn generate_store_task(instance: Instance) -> Result<StoreTaskInfo>;
}

pub struct StoreServiceImpl<D, V, S, C, P, R> {
    delivery: PhantomData<D>,
    instance_trait: PhantomData<V>,
    instance_dao: PhantomData<S>,
    thing_define_cache_trait: PhantomData<C>,
    dispatch_service: PhantomData<P>,
    route: PhantomData<R>,
}

impl<D, V, S, C, P, R> StoreServiceTrait for StoreServiceImpl<D, V, S, C, P, R>
    where D: DeliveryServiceTrait,
          V: InstanceServiceTrait,
          S: InstanceDao,
          C: ThingDefineCacheTrait,
          P: DispatchTrait,
          R: RouteServiceTrait
{
    /// born an instance which is the beginning of the changes.
    fn input(mut instance: Instance) -> Result<u128>
    {
        let uuid = V::verify(&mut instance, Root::Business)?;
        let task = Self::generate_store_task(instance)?;
        Self::send_store_task(task)?;
        Ok(uuid)
    }

    fn receive_store_task(carrier: Carrier<StoreTaskInfo>) {
        if let Err(err) = Self::save(carrier.clone()) {
            D::move_to_err(err, carrier)
        };
    }

    fn send_store_task(task: StoreTaskInfo) -> Result<()> {
        // get route info
        let biz = task.instance.data.thing.key.clone();
        let carrier = D::create_carrier(task, biz, DataType::Store as u8)?;
        // to delivery
        D::send_carrier(&CHANNEL_STORE.sender, carrier);
        Ok(())
    }

    /// generate `StoreTaskInfo` include route information.
    /// `Err` on environment error
    fn generate_store_task(instance: Instance) -> Result<StoreTaskInfo> {
        let target = R::get_route(&instance)?;
        // save to delivery to make it can redo
        let task = StoreTaskInfo { instance, upstream: None, target };
        Ok(task)
    }
}

impl<D, V, S, C, P, R> StoreServiceImpl<D, V, S, C, P, R>
    where D: DeliveryServiceTrait, C: ThingDefineCacheTrait, P: DispatchTrait, S: InstanceDao
{
    /// save to db and handle duplicated data
    fn save(carrier: Carrier<StoreTaskInfo>) -> Result<u128> {
        let id = carrier.instance.id;
        let result = S::insert(&carrier.instance);
        match result {
            Ok(_) => {
                D::send_carrier(&CHANNEL_DISPATCH.sender, carrier);
                Ok(id)
            }
            Err(NatureError::DaoDuplicated) => {
                Self::handle_duplicated(carrier)?;
                Ok(id)
            }
            Err(err) => Err(err)
        }
    }

    fn handle_duplicated(carrier: Carrier<StoreTaskInfo>) -> Result<()> {
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

pub type StoreService = StoreServiceImpl<DeliveryService, InstanceImpl, TableInstance, ThingDefineCacheImpl, DispatchTask, RouteService>;
