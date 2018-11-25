use flow::sequential::SequentialServiceImpl;
use nature_db::*;
use nature_db::service::*;
use std::rc::Rc;
use super::*;

lazy_static! {
    pub static ref SVC_NATURE : ControllerImpl = ControllerImpl::new();
}

pub struct ControllerImpl {
    pub store_svc: Rc<StoreServiceTrait>,
    pub delivery_svc: Rc<TaskServiceTrait>,
    pub delivery_dao: Rc<TaskDaoTrait>,
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
        let dao_delivery = Rc::new(TaskDaoImpl {});
        let svc_delivery = Rc::new(TaskServiceImpl { table_delivery: dao_delivery.clone() });
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
}

unsafe impl Sync for ControllerImpl {}
