use flow::delivery::DeliveryServiceTrait;
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
    fn generate_store_task(instance: &Instance) -> Result<StoreTaskInfo>;
    fn save(carrier: &Carrier<StoreTaskInfo>) -> Result<u128>;
}

pub struct StoreServiceImpl<D, S, R> {
    delivery: PhantomData<D>,
    instance_dao: PhantomData<S>,
    route: PhantomData<R>,
}

impl<D, S, R> StoreServiceTrait for StoreServiceImpl<D, S, R>
    where
        D: DeliveryServiceTrait,
        S: InstanceDaoTrait,
        R: RouteServiceTrait
{
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

    /// save to db and handle duplicated data
    fn save(carrier: &Carrier<StoreTaskInfo>) -> Result<u128> {
        let id = carrier.instance.id;
        debug!("save instance for `Thing` {:?}, id: {:?}", carrier.instance.thing.key, id);
        let result = S::insert(&carrier.instance);
        match result {
            Ok(_) => {
                D::send_carrier(&CHANNEL_DISPATCH.sender, carrier.clone());
                Ok(id)
            }
            Err(err) => match err {
                NatureError::DaoDuplicated(_) => {
                    // delivery will be retry by back-end.service
                    Ok(id)
                }
                _ => Err(err)
            }
        }
    }
}

