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
    fn do_store_task(carrier: Carrier<StoreTaskInfo>);
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
          S: InstanceDaoTrait,
          C: ThingDefineCacheTrait,
          P: DispatchServiceTrait,
          R: RouteServiceTrait
{
    /// born an instance which is the beginning of the changes.
    fn input(mut instance: Instance) -> Result<u128>
    {
        debug!("get instance: {:?}", instance);
        instance.data.thing.thing_type = ThingType::Business;
        let uuid = V::verify(&mut instance)?;
        let task = Self::generate_store_task(instance)?;
        Self::send_store_task(task)?;
        Ok(uuid)
    }

    /// generate `StoreTaskInfo` include route information.
    /// `Err` on environment error
    fn generate_store_task(instance: Instance) -> Result<StoreTaskInfo> {
        debug!("generate store task for instance id : {:?}", instance.id);
        let target = R::get_route(&instance)?;
        debug!("routes info for instance : {:?}", target);
        // save to delivery to make it can redo
        let task = StoreTaskInfo { instance, upstream: None, target };
        Ok(task)
    }

    fn do_store_task(carrier: Carrier<StoreTaskInfo>) {
        debug!("------------------do_store_task------------------------");
        if let Err(err) = Self::save(carrier.clone()) {
            D::move_to_err(err, carrier)
        };
    }

    fn send_store_task(task: StoreTaskInfo) -> Result<()> {
        // get route info
        let biz = task.instance.data.thing.key.clone();
        debug!("create carrier for store task, the instance id is : {:?}", task.instance.id);
        let carrier = D::create_carrier(task, biz, DataType::Store as u8)?;
        // send to this service again to unify the store process.
        D::send_carrier(&CHANNEL_STORE.sender, carrier);
        Ok(())
    }
}

impl<D, V, S, C, P, R> StoreServiceImpl<D, V, S, C, P, R>
    where D: DeliveryServiceTrait, C: ThingDefineCacheTrait, P: DispatchServiceTrait, S: InstanceDaoTrait
{
    /// save to db and handle duplicated data
    fn save(carrier: Carrier<StoreTaskInfo>) -> Result<u128> {
        let id = carrier.instance.id;
        debug!("save instance for id: {:?}", id);
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
            D::finish_carrier(carrier.id)?;
            Ok(())
        }
    }
}
