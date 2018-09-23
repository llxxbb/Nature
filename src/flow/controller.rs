use nature_common::*;
use nature_db::*;
use std::marker::PhantomData;
use std::thread::JoinHandle;
use super::*;

type StoreService = StoreServiceImpl<DeliveryService, InstanceDaoImpl, RouteService>;
type DeliveryService = DeliveryServiceImpl<DeliveryDaoImpl>;
pub type ConvertService = ConvertServiceImpl<PlanService, DeliveryService, StoreService, CallOutImpl<LocalExecutorImpl>, InstanceServiceImpl>;
pub type DispatchService = DispatchServiceImpl<DeliveryService, ConvertService>;
type RouteService = RouteServiceImpl<DeliveryService, OneStepFlowCacheImpl<OneStepFlowDaoImpl>>;
pub type SequentialService = SequentialServiceImpl<DeliveryService, StoreService, InstanceServiceImpl>;
type PlanService = PlanServiceImpl<StorePlanDaoImpl>;
pub type ParallelService = ParallelServiceImpl<DeliveryService, StoreService>;

pub type Controller = ControllerImpl<StoreService, DeliveryService, InstanceServiceImpl, ConvertService, SequentialService, ParallelService>;

pub trait ControllerTrait {
    fn input(instance: Instance) -> Result<u128>;
    fn do_store_task(carrier: Carrier<StoreTaskInfo>);
    fn callback(delayed: DelayedInstances) -> Result<()>;
    fn serial(batch: SerialBatchInstance) -> Result<()>;
    fn parallel(batch: ParallelBatchInstance) -> Result<()>;
}

pub struct ControllerImpl<S, D, V, C, BS, BP> {
    store_svc: PhantomData<S>,
    delivery_svc: PhantomData<D>,
    ins_svc: PhantomData<V>,
    converter_svc: PhantomData<C>,
    batch_serial_svc: PhantomData<BS>,
    batch_parallel_svc: PhantomData<BP>,
}

impl<S, D, V, C, BS, BP> ControllerTrait for ControllerImpl<S, D, V, C, BS, BP>
    where S: StoreServiceTrait, D: DeliveryServiceTrait, V: InstanceServiceTrait,
          C: ConvertServiceTrait, BS: SequentialTrait, BP: ParallelServiceTrait
{
    /// born an instance which is the beginning of the changes.
    fn input(mut instance: Instance) -> Result<u128> {
        instance.data.thing.thing_type = ThingType::Business;
        let uuid = V::verify(&mut instance)?;
        let task = S::generate_store_task(&instance)?;
        let carrier = D::create_carrier(task, &instance.data.thing.key, DataType::Store as u8)?;
        Self::store(&carrier)?;
        Ok(uuid)
    }

    fn do_store_task(carrier: Carrier<StoreTaskInfo>) {
        debug!("------------------do_store_task------------------------");
        let _ = Self::store(&carrier);
    }

    fn callback(delayed: DelayedInstances) -> Result<()> {
        C::submit_callback(delayed)
    }

    fn serial(batch: SerialBatchInstance) -> Result<()> {
        BS::submit_serial(batch)
    }

    fn parallel(batch: ParallelBatchInstance) -> Result<()> {
        BP::submit_parallel(batch)
    }
}


impl<S, D, V, C, BS, BP> ControllerImpl<S, D, V, C, BS, BP>
    where S: StoreServiceTrait, D: DeliveryServiceTrait
{
    pub fn start() -> Vec<JoinHandle<()>> {
        start_receive_threads()
    }
    fn store(carrier: &Carrier<StoreTaskInfo>) -> Result<()> {
        if let Err(err) = S::save(carrier) {
            D::move_to_err(err.clone(), carrier);
            Err(err)
        } else {
            Ok(())
        }
    }
}

