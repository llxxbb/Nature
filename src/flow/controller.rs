use std::rc::Rc;

use crate::flow::*;
use nature_db::service::*;

lazy_static! {
    pub static ref SVC_NATURE : ControllerImpl = ControllerImpl::new();
}

pub struct ControllerImpl {
    pub store_svc: Rc<StoreServiceTrait>,
    pub task_svc: Rc<TaskServiceTrait>,
    pub batch_serial_svc: Rc<SequentialTrait>,
    pub batch_parallel_svc: Rc<ParallelServiceTrait>,
    pub converter_svc: Rc<ConvertServiceTrait>,
    pub plan_svc: Rc<PlanServiceTrait>,
}

impl ControllerImpl {
    pub fn new() -> Self {
        let cache_define = SVC_DB.thing_define.clone();
        let svc_task = Rc::new(TaskServiceImpl {});
        let svc_store = Rc::new(StoreServiceImpl {
            route: Rc::new(RouteServiceImpl {
                task_service: svc_task.clone(),
            }),
            task_svc: svc_task.clone(),
        });

        ControllerImpl {
            store_svc: svc_store.clone(),
            batch_serial_svc: Rc::new(SequentialServiceImpl {
                svc_task: svc_task.clone(),
                store: svc_store.clone(),
            }),
            batch_parallel_svc: Rc::new(ParallelServiceImpl {
                task_svc: svc_task.clone(),
                store: svc_store.clone(),
            }),
            converter_svc: Rc::new(ConvertServiceImpl {
                svc_task: svc_task.clone(),
                caller: Rc::new(CallOutImpl {
                    local_rust: Rc::new(LocalExecutorImpl {}),
                    http_caller: Rc::new(HttpExecutorImpl {})
                }),
                svc_define: cache_define.clone(),
            }),
            task_svc: svc_task.clone(),
            plan_svc: Rc::new(PlanServiceImpl { dao: Rc::new(StorePlanDaoImpl {}) }),
        }
    }
}

impl Default for ControllerImpl {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for ControllerImpl {}
