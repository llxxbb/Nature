use flow::sequential::SequentialServiceImpl;
use nature_common::*;
use nature_db::*;
use nature_db::service::*;
use std::rc::Rc;
use super::*;

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
    pub fn input(&self, instance: Instance) -> Result<u128> {
        self.store_svc.input(instance)
    }

    pub fn callback(&self, delayed: DelayedInstances) -> Result<()> {
        self.converter_svc.callback(delayed)
    }

    pub fn serial(&self, batch: SerialBatchInstance) -> Result<()> {
        self.batch_serial_svc.one_by_one(&batch)
    }

    pub fn parallel(&self, batch: ParallelBatchInstance) -> Result<()> {
        self.batch_parallel_svc.parallel(batch)
    }

    pub fn channel_stored(&self, task: &StoreTaskInfo, carrier: RawDelivery) {
        if task.mission.is_none() {
            let _ = self.delivery_dao.delete(&carrier.id);
            return;
        }
        let converters = match self.converter_svc.generate_converter_info(task) {
            Ok(new) => new,
            Err(err) => match err {
                NatureError::DaoEnvironmentError(_) => return,
                _ => {
                    self.delivery_dao.raw_to_error(&err, &carrier);
                    return;
                }
            }
        };
        let biz = &task.instance.thing.key;
        let raws: Vec<RawDelivery> = converters.iter().map(|x| x.1).collect();
        if let Ok(_) = self.delivery_svc.create_batch_and_finish_carrier(&raws, &carrier.id) {
            debug!("will dispatch {} convert tasks for `Thing` : {:?}", converters.len(), biz);
            for task in converters {
                CHANNEL_CONVERT.sender.lock().unwrap().send(task);
            }
        };
    }

    pub fn channel_converted(&self, converted: Converted) {
        if let Ok(plan) = self.plan_svc.new(&converted.done_task.content.data, &converted.converted) {
            self.prepare_to_store(&converted.done_task, plan);
        }
    }
    fn prepare_to_store(&self, carrier: &RawDelivery, plan: PlanInfo) {
        let mut store_infos: Vec<RawDelivery> = Vec::new();
        for instance in plan.plan.iter() {
            match self.store_svc.generate_store_task(instance) {
                Ok(task) => {
                    match RawDelivery::new(task, &plan.to.key, DataType::Store as i16) {
                        Ok(x) => store_infos.push(x),
                        Err(e) => {
                            error!("{}", e);
                            self.delivery_dao.raw_to_error(&e, carrier);
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
        if let Ok(_) = self.delivery_svc.create_batch_and_finish_carrier(&store_infos, &carrier) {
            for task in store_infos {
                self.delivery_svc.send_carrier(&CHANNEL_STORE.sender, task)
            }
        }
    }
}

unsafe impl Sync for ControllerImpl {}
