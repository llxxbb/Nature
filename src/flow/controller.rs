use nature_common::*;
use nature_db::*;
use std::marker::PhantomData;
use std::thread::JoinHandle;
use super::*;

type DeliveryService = DeliveryServiceImpl<DeliveryDaoImpl>;
pub type StoreService = StoreServiceImpl<InstanceDaoImpl, RouteService, DeliveryService, InstanceServiceImpl>;
type RouteService = RouteServiceImpl<DeliveryService, OneStepFlowCacheImpl<OneStepFlowDaoImpl>>;
pub type ConvertService = ConvertServiceImpl<DeliveryService, CallOutImpl<LocalExecutorImpl>, InstanceServiceImpl>;
type PlanService = PlanServiceImpl<StorePlanDaoImpl>;
pub type SequentialService = SequentialServiceImpl<DeliveryService, StoreService, InstanceServiceImpl>;
pub type ParallelService = ParallelServiceImpl<DeliveryService, StoreService>;

pub type Controller = ControllerImpl<StoreService, DeliveryService, SequentialService, ParallelService, ConvertService, PlanService>;

pub struct ControllerImpl<STORE, DELIVERY, BS, BP, CONVERTER, PLAN> {
    store_svc: PhantomData<STORE>,
    delivery_svc: PhantomData<DELIVERY>,
    batch_serial_svc: PhantomData<BS>,
    batch_parallel_svc: PhantomData<BP>,
    converter_svc: PhantomData<CONVERTER>,
    plan_svc: PhantomData<PLAN>,
}

impl<STORE, DELIVERY, BS, BP, CONVERTER, PLAN> ControllerImpl<STORE, DELIVERY, BS, BP, CONVERTER, PLAN>
    where STORE: StoreServiceTrait, DELIVERY: DeliveryServiceTrait, PLAN: PlanServiceTrait,
          BS: SequentialTrait, BP: ParallelServiceTrait, CONVERTER: ConvertServiceTrait
{
    /// born an instance which is the beginning of the changes.
    pub fn input(instance: Instance) -> Result<u128> {
        STORE::input(instance)
    }

    pub fn callback(delayed: DelayedInstances) -> Result<()> {
        CONVERTER::callback(delayed)
    }

    pub fn serial(batch: SerialBatchInstance) -> Result<()> {
        BS::one_by_one(batch)
    }

    pub fn parallel(batch: ParallelBatchInstance) -> Result<()> {
        BP::parallel(batch)
    }

    pub fn channel_stored(carrier: Carrier<StoreTaskInfo>) {
        if carrier.content.data.mission.is_none() {
            let _ = DELIVERY::finish_carrier(carrier.id);
            return;
        }
        let converters = match CONVERTER::generate_converter_info(&carrier) {
            Ok(new) => new,
            Err(err) => match err {
                NatureError::DaoEnvironmentError(_) => return,
                _ => {
                    DELIVERY::move_to_err(&err, &carrier);
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

    pub fn channel_converted(converted: Converted) {
        if let Ok(plan) = PLAN::new(&converted.done_task.content.data, &converted.converted) {
            Self::prepare_to_store(&converted.done_task, plan);
        }
    }
    pub fn start() -> Vec<JoinHandle<()>> {
        start_receive_threads()
    }

    fn prepare_to_store(carrier: &Carrier<ConverterInfo>, plan: PlanInfo) {
        let mut store_infos: Vec<Carrier<StoreTaskInfo>> = Vec::new();
        for instance in plan.plan.iter() {
            match STORE::generate_store_task(instance) {
                Ok(task) => {
                    match DELIVERY::new_carrier(task, &plan.to.key, DataType::Store as u8) {
                        Ok(x) => store_infos.push(x),
                        Err(e) => {
                            error!("{}", e);
                            DELIVERY::move_to_err(&e, carrier);
                            return;
                        }
                    }
                }
                // break process will environment error occurs.
                Err(e) => {
                    error!("{}", e);
                    return;
                }
            }
        }
        if let Ok(_) = DELIVERY::create_batch_and_finish_carrier(&store_infos, &carrier.to_owned()) {
            for task in store_infos {
                DELIVERY::send_carrier(&CHANNEL_STORE.sender, task)
            }
        }
    }
}

