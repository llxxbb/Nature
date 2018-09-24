use nature_common::*;
use nature_db::*;
use std::marker::PhantomData;
use std::thread::JoinHandle;
use super::*;

type DeliveryService = DeliveryServiceImpl<DeliveryDaoImpl>;
pub type StoreService = StoreServiceImpl<InstanceDaoImpl, RouteService, DeliveryService, InstanceServiceImpl>;
type RouteService = RouteServiceImpl<DeliveryService, OneStepFlowCacheImpl<OneStepFlowDaoImpl>>;
pub type DispatchService = DispatchServiceImpl<DeliveryService, ConvertService>;
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
}


impl<STORE, DELIVERY, BS, BP, CONVERTER> ControllerImpl<STORE, DELIVERY, BS, BP, CONVERTER>
{
    pub fn start() -> Vec<JoinHandle<()>> {
        start_receive_threads()
    }
}

