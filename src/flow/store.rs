use flow::route::RouteServiceTrait;
use std::marker::PhantomData;
use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StoreTaskInfo {
    pub instance: Instance,
    /// save outside has non converter info.
    pub upstream: Option<ConverterInfo>,
    pub mission: Option<Vec<Mission>>,
}

pub trait StoreServiceTrait {
    fn input(instance: Instance) -> Result<u128>;
    fn store(carrier: Carrier<StoreTaskInfo>);
    fn generate_store_task(instance: &Instance) -> Result<StoreTaskInfo>;
}

pub struct StoreServiceImpl<S, R, D, V> {
    instance_dao: PhantomData<S>,
    route: PhantomData<R>,
    delivery: PhantomData<D>,
    ins_svc: PhantomData<V>,
}

impl<S, R, D, V> StoreServiceTrait for StoreServiceImpl<S, R, D, V>
    where
        S: InstanceDaoTrait,
        R: RouteServiceTrait,
        D: DeliveryServiceTrait,
        V: InstanceServiceTrait
{
    fn input(mut instance: Instance) -> Result<u128> {
        instance.data.thing.thing_type = ThingType::Business;
        let uuid = V::verify(&mut instance)?;
        let task = Self::generate_store_task(&instance)?;
        let carrier = D::create_carrier(task, &instance.data.thing.key, DataType::Store as u8)?;
        Self::do_task(&carrier)?;
        Ok(uuid)
    }

    fn store(carrier: Carrier<StoreTaskInfo>) {
        let _ = Self::do_task(&carrier);
    }

    /// generate `StoreTaskInfo` include route information.
    /// `Err` on environment error
    fn generate_store_task(instance: &Instance) -> Result<StoreTaskInfo> {
//        let key = &instance.thing.key;
        let target = R::get_route(instance)?;
        // save to delivery to make it can redo
        let task = StoreTaskInfo {
            instance: instance.clone(),
            upstream: None,
            mission: target,
        };
        Ok(task)
    }
}

impl<S, R, D, V> StoreServiceImpl<S, R, D, V>
    where
        D: DeliveryServiceTrait,
        S: InstanceDaoTrait
{
    /// save to db and handle duplicated data
    fn save(carrier: &Carrier<StoreTaskInfo>) -> Result<u128> {
        let id = carrier.instance.id;
        debug!("save instance for `Thing` {:?}, id: {:?}", carrier.instance.thing.key, id);
        let result = S::insert(&carrier.instance);
        match result {
            Ok(_) => Ok(id),
            Err(err) => match err {
                NatureError::DaoDuplicated(_) => Ok(id),
                _ => Err(err)
            }
        }
    }

    fn do_task(carrier: &Carrier<StoreTaskInfo>) -> Result<()> {
        debug!("------------------do_store_task------------------------");
        if let Err(err) = Self::save(carrier) {
            D::move_to_err(err.clone(), carrier);
            Err(err)
        } else {
            D::send_carrier(&CHANNEL_DISPATCH.sender, carrier.clone());
            Ok(())
        }
    }
}