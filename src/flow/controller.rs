use std::rc::Rc;

use crate::flow::*;

lazy_static! {
    pub static ref SVC_NATURE : ControllerImpl = ControllerImpl::new();
}

pub struct ControllerImpl {
    pub task_svc: Rc<TaskServiceTrait>,
    pub batch_serial_svc: Rc<SequentialTrait>,
    pub batch_parallel_svc: Rc<ParallelServiceTrait>,
}

impl ControllerImpl {
    pub fn new() -> Self {
        let svc_task = Rc::new(TaskServiceImpl {});

        ControllerImpl {
            batch_serial_svc: Rc::new(SequentialServiceImpl {
                svc_task: svc_task.clone(),
            }),
            batch_parallel_svc: Rc::new(ParallelServiceImpl {
                task_svc: svc_task.clone(),
            }),
            task_svc: svc_task.clone(),
        }
    }
}

impl Default for ControllerImpl {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for ControllerImpl {}
