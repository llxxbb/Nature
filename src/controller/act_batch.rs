use nature_common::{Instance, NatureError, Result};
use nature_db::{MetaCacheImpl, MetaDaoImpl, Mission, RawTask, RelationCacheImpl, RelationDaoImpl, TaskDaoImpl, TaskType};
use nature_db::flow_tool::{context_check, state_check};

use crate::actor::{ACT_STORE, MsgForTask};
use crate::task::TaskForStore;

pub fn channel_parallel(task: MsgForTask<Vec<Instance>>) {
    if let Err(e) = inner_batch(&task) {
        let _ = TaskDaoImpl::raw_to_error(&e, &task.1);
    }
}

fn inner_batch(task: &MsgForTask<Vec<Instance>>) -> Result<()> {
    let mut tuple: Vec<(TaskForStore, RawTask)> = Vec::new();
    let from = &task.0[0].meta;
    match RelationCacheImpl::get(from, RelationDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get) {
        Ok(relations) => {
            for instance in task.0.iter() {
                let mission = Mission::get_by_instance(&instance, &relations, context_check, state_check);
                let f_meta = MetaCacheImpl::get(from, MetaDaoImpl::get)?;
                let task = TaskForStore::new(instance.clone(), mission, None, f_meta.need_cache());
                let raw = RawTask::new(&task, &instance.get_key(), TaskType::Store as i8, &instance.meta)?;
                match TaskDaoImpl::insert(&raw) {
                    Ok(_) => tuple.push((task, raw)),
                    Err(NatureError::EnvironmentError(_)) => return Ok(()),
                    Err(e) => return Err(e)
                }
            }
            for c in tuple {
                ACT_STORE.do_send(MsgForTask(c.0, c.1));
            }
            let _ = TaskDaoImpl::finish_task(&task.1.task_id);
            Ok(())
        }
        Err(NatureError::EnvironmentError(_)) => Ok(()),
        Err(e) => Err(e)
    }
}
