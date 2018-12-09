use std::rc::Rc;

use flow::sequential::SequentialServiceImpl;
use nature_db::*;
use nature_db::service::*;

use super::*;

lazy_static! {
    pub static ref SVC_NATURE : ControllerImpl = ControllerImpl::new();
}

pub struct ControllerImpl {
    pub store_svc: Rc<StoreServiceTrait>,
    pub task_svc: Rc<TaskServiceTrait>,
    pub task_dao: Rc<TaskDaoTrait>,
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
        let dao_task = Rc::new(TaskDaoImpl {});
        let svc_task = Rc::new(TaskServiceImpl { table_task: dao_task.clone() });
        let svc_store = Rc::new(StoreServiceImpl {
            instance_dao: dao_instance.clone(),
            route: Rc::new(RouteServiceImpl {
                task_service: svc_task.clone(),
                one_step_flow_cache: Rc::new(OneStepFlowCacheImpl { dao: Rc::new(OneStepFlowDaoImpl {}) }),
            }),
            task_svc: svc_task.clone(),
            task_dao: dao_task.clone(),
            svc_instance: svc_instance.clone(),
        });

        ControllerImpl {
            store_svc: svc_store.clone(),
            batch_serial_svc: Rc::new(SequentialServiceImpl {
                svc_task: svc_task.clone(),
                dao_task: dao_task.clone(),
                store: svc_store.clone(),
                svc_instance: svc_instance.clone(),
                dao_instance: dao_instance.clone(),
            }),
            batch_parallel_svc: Rc::new(ParallelServiceImpl {
                task_svc: svc_task.clone(),
                task_dao: dao_task.clone(),
                store: svc_store.clone(),
            }),
            converter_svc: Rc::new(ConvertServiceImpl {
                svc_task: svc_task.clone(),
                dao_task: dao_task.clone(),
                caller: Rc::new(CallOutImpl { local_rust: Rc::new(LocalExecutorImpl {}) }),
                svc_define: cache_define.clone(),
                dao_instance: dao_instance.clone(),
                svc_instance: svc_instance.clone(),
            }),
            task_svc: svc_task.clone(),
            plan_svc: Rc::new(PlanServiceImpl { dao: Rc::new(StorePlanDaoImpl {}) }),
            task_dao: dao_task.clone(),
        }
    }
}

impl Default for ControllerImpl {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for ControllerImpl {}
