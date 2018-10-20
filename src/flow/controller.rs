use flow::sequential::SequentialServiceImpl;
use nature_common::*;
use nature_db::*;
use nature_db::service::*;
use serde::Deserialize;
use serde_json;
use std::convert::TryFrom;
use std::rc::Rc;
use super::*;

lazy_static! {
    static ref SVC_NATURE : ControllerImpl = ControllerImpl::new();
}

pub struct ControllerImpl {
    pub store_svc: Rc<StoreServiceTrait>,
    pub delivery_svc: Rc<DeliveryServiceTrait>,
    pub delivery_dao: Rc<DeliveryDaoTrait>,
    pub batch_serial_svc: Rc<SequentialTrait>,
    pub batch_parallel_svc: Rc<ParallelServiceTrait>,
    pub converter_svc: Rc<ConvertServiceTrait>,
    pub plan_svc: Rc<PlanServiceTrait>,
}

impl ControllerImpl {
    pub fn new() -> Self {
        let cache_define = SVC_DB.thing_define.clone();
        let dao_instance = Rc::new(InstanceDaoImpl {});
        let svc_instance = Rc::new(InstanceServiceImpl { define_cache: cache_define.clone() });
        let dao_delivery = Rc::new(DeliveryDaoImpl {});
        let svc_delivery = Rc::new(DeliveryServiceImpl { table_delivery: dao_delivery.clone() });
        let svc_store = Rc::new(StoreServiceImpl {
            instance_dao: dao_instance.clone(),
            route: Rc::new(RouteServiceImpl {
                delivery_service: svc_delivery.clone(),
                one_step_flow_cache: Rc::new(OneStepFlowCacheImpl { dao: Rc::new(OneStepFlowDaoImpl {}) }),
            }),
            delivery_svc: svc_delivery.clone(),
            delivery_dao: dao_delivery.clone(),
            svc_instance: svc_instance.clone(),
        });

        ControllerImpl {
            store_svc: svc_store.clone(),
            batch_serial_svc: Rc::new(SequentialServiceImpl {
                svc_delivery: svc_delivery.clone(),
                dao_delivery: dao_delivery.clone(),
                store: svc_store.clone(),
                svc_instance: svc_instance.clone(),
                dao_instance: dao_instance.clone(),
            }),
            batch_parallel_svc: Rc::new(ParallelServiceImpl {
                delivery_svc: svc_delivery.clone(),
                delivery_dao: dao_delivery.clone(),
                store: svc_store.clone(),
            }),
            converter_svc: Rc::new(ConvertServiceImpl {
                svc_delivery: svc_delivery.clone(),
                dao_delivery: dao_delivery.clone(),
                caller: Rc::new(CallOutImpl { local_rust: Rc::new(LocalExecutorImpl {}) }),
                svc_define: cache_define.clone(),
                dao_instance: dao_instance.clone(),
                svc_instance: svc_instance.clone(),
            }),
            delivery_svc: svc_delivery.clone(),
            plan_svc: Rc::new(PlanServiceImpl { dao: Rc::new(StorePlanDaoImpl {}) }),
            delivery_dao: dao_delivery.clone(),
        }
    }

    /// born an instance which is the beginning of the changes.
    pub fn input(instance: Instance) -> Result<u128> {
        SVC_NATURE.store_svc.input(instance)
    }

    pub fn callback(delayed: DelayedInstances) -> Result<()> {
        SVC_NATURE.converter_svc.callback(delayed)
    }

    pub fn redo_task(raw: RawTask) -> Result<()> {
        match TaskType::try_from(raw.data_type)? {
            TaskType::Store => Self::send_to_channel::<StoreTaskInfo>(&raw, &CHANNEL_STORED)?,
            TaskType::Convert => Self::send_to_channel::<ConverterInfo>(&raw, &CHANNEL_CONVERT)?,
            TaskType::ParallelBatch => Self::send_to_channel::<ParallelBatchInstance>(&raw, &CHANNEL_PARALLEL)?,
            TaskType::QueueBatch => Self::send_to_channel::<SerialBatchInstance>(&raw, &CHANNEL_SERIAL)?,
        }
        Ok(())
    }

    fn send_to_channel<'a, T: Deserialize<'a>>(raw: &'a RawTask, channel: &Channel<(T, RawTask)>) -> Result<()> {
        let task: T = serde_json::from_str(&raw.data)?;
        let _ = channel.sender.lock().unwrap().send((task, raw.clone()));
        Ok(())
    }

    pub fn channel_serial(task: (SerialBatchInstance, RawTask)) {
        SVC_NATURE.batch_serial_svc.do_serial_task(task.0, &task.1)
    }
    pub fn serial(batch: SerialBatchInstance) -> Result<()> {
        SVC_NATURE.batch_serial_svc.one_by_one(&batch)
    }

    pub fn channel_parallel(task: (ParallelBatchInstance, RawTask)) {
        SVC_NATURE.batch_parallel_svc.do_parallel_task(task.0, task.1)
    }
    pub fn parallel(batch: ParallelBatchInstance) -> Result<()> {
        SVC_NATURE.batch_parallel_svc.parallel(batch)
    }

    pub fn channel_store(store: (StoreTaskInfo, RawTask)) {
        let _ = SVC_NATURE.store_svc.do_task(&store.0, &store.1);
    }
    pub fn channel_stored(store: (StoreTaskInfo, RawTask)) {
        if store.0.mission.is_none() {
            let _ = SVC_NATURE.delivery_dao.delete(&&store.1.task_id);
            return;
        }
        let converters = match SVC_NATURE.converter_svc.generate_converter_info(&store.0) {
            Ok(new) => new,
            Err(err) => match err {
                NatureError::DaoEnvironmentError(_) => return,
                _ => {
                    let _ = SVC_NATURE.delivery_dao.raw_to_error(&err, &store.1);
                    return;
                }
            }
        };
        let biz = &store.0.instance.thing.key;
        let raws: Vec<RawTask> = converters.iter().map(|x| x.1.clone()).collect();
        if let Ok(_) = SVC_NATURE.delivery_svc.create_batch_and_finish_carrier(&raws, &store.1.task_id) {
            debug!("will dispatch {} convert tasks for `Thing` : {:?}", converters.len(), biz);
            for task in converters {
                let _ = CHANNEL_CONVERT.sender.lock().unwrap().send(task);
            }
        };
    }

    pub fn channel_convert(task: (ConverterInfo, RawTask)) {
        SVC_NATURE.converter_svc.convert(&task.0, &task.1);
    }
    pub fn channel_converted(task: (ConverterInfo, Converted)) {
        if let Ok(plan) = SVC_NATURE.plan_svc.new(&task.0, &task.1.converted) {
            SVC_NATURE.prepare_to_store(&task.1.done_task, plan);
        }
    }
    fn prepare_to_store(&self, carrier: &RawTask, plan: PlanInfo) {
        let mut store_infos: Vec<RawTask> = Vec::new();
        let mut t_d: Vec<(StoreTaskInfo, RawTask)> = Vec::new();
        for instance in plan.plan.iter() {
            match self.store_svc.generate_store_task(instance) {
                Ok(task) => {
                    match RawTask::new(&task, &plan.to.key, TaskType::Store as i16) {
                        Ok(x) => {
                            store_infos.push(x.clone());
                            t_d.push((task, x))
                        }
                        Err(e) => {
                            error!("{}", e);
                            let _ = self.delivery_dao.raw_to_error(&e, carrier);
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
        if let Ok(_) = self.delivery_svc.create_batch_and_finish_carrier(&store_infos, &carrier.task_id) {
            for task in t_d {
                let _ = CHANNEL_STORE.sender.lock().unwrap().send(task);
            }
        }
    }
}

unsafe impl Sync for ControllerImpl {}
