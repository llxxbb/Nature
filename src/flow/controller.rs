use nature_common::*;
use nature_db::*;
use std::marker::PhantomData;
use std::thread::JoinHandle;
use super::*;

type DeliveryService = DeliveryServiceImpl<DeliveryDaoImpl>;
pub type StoreService = StoreServiceImpl<InstanceDaoImpl, RouteService, DeliveryService, InstanceServiceImpl>;
type RouteService = RouteServiceImpl<DeliveryService, OneStepFlowCacheImpl<OneStepFlowDaoImpl>>;
pub type ConvertService = ConvertServiceImpl<PlanService, DeliveryService, StoreService, CallOutImpl<LocalExecutorImpl>, InstanceServiceImpl>;
type PlanService = PlanServiceImpl<StorePlanDaoImpl>;
pub type SequentialService = SequentialServiceImpl<DeliveryService, StoreService, InstanceServiceImpl>;
pub type ParallelService = ParallelServiceImpl<DeliveryService, StoreService>;

pub type Controller = ControllerImpl<StoreService, DeliveryService, SequentialService, ParallelService, ConvertService>;

pub trait ControllerTrait {
    fn input(instance: Instance) -> Result<u128>;
    fn callback(delayed: DelayedInstances) -> Result<()>;
    fn serial(batch: SerialBatchInstance) -> Result<()>;
    fn parallel(batch: ParallelBatchInstance) -> Result<()>;
    fn channel_stored(carrier: Carrier<StoreTaskInfo>);
}

pub struct ControllerImpl<STORE, DELIVERY, BS, BP, CONVERTER> {
    store_svc: PhantomData<STORE>,
    delivery_svc: PhantomData<DELIVERY>,
    batch_serial_svc: PhantomData<BS>,
    batch_parallel_svc: PhantomData<BP>,
    converter_svc: PhantomData<CONVERTER>,
}

impl<STORE, DELIVERY, BS, BP, CONVERTER> ControllerTrait for ControllerImpl<STORE, DELIVERY, BS, BP, CONVERTER>
    where STORE: StoreServiceTrait, DELIVERY: DeliveryServiceTrait,
          BS: SequentialTrait, BP: ParallelServiceTrait, CONVERTER: ConvertServiceTrait
{
    /// born an instance which is the beginning of the changes.
    fn input(instance: Instance) -> Result<u128> {
        STORE::input(instance)
    }

    fn callback(delayed: DelayedInstances) -> Result<()> {
        CONVERTER::callback(delayed)
    }

    fn serial(batch: SerialBatchInstance) -> Result<()> {
        BS::one_by_one(batch)
    }

    fn parallel(batch: ParallelBatchInstance) -> Result<()> {
        BP::parallel(batch)
    }

    fn channel_stored(carrier: Carrier<StoreTaskInfo>) {
        if carrier.content.data.mission.is_none() {
            let _ = DELIVERY::finish_carrier(carrier.id);
            return;
        }
        let converters = match CONVERTER::generate_converter_info(&carrier) {
            Ok(new) => new,
            Err(err) => match err {
                NatureError::DaoEnvironmentError(_) => return,
                _ => {
                    DELIVERY::move_to_err(err, &carrier);
                    return;
                }
            }
        };
        let biz = &carrier.instance.thing.key;
        if let Ok(_) = DELIVERY::create_batch_and_finish_carrier(&converters, &carrier) {
            debug!("will dispatch {} convert tasks for `Thing` : {:?}", converters.len(), biz);
            for task in converters {
                DELIVERY::send_carrier(&CHANNEL_CONVERT.sender, task)
            }
        };
    }
}


impl<STORE, DELIVERY, BS, BP, CONVERTER> ControllerImpl<STORE, DELIVERY, BS, BP, CONVERTER>
{
    pub fn start() -> Vec<JoinHandle<()>> {
        start_receive_threads()
    }
}

