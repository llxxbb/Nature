use nature_common::{Instance, MetaType, Result};
use nature_db::{MetaCacheImpl, MG, Mission, RawTask, RelationCacheImpl, RelationDaoImpl, TaskDaoImpl, TaskType};
use nature_db::flow_tool::{context_check, state_check};

use crate::controller::channel_store;
use crate::task::TaskForStore;

pub fn channel_batch(instances: Vec<Instance>, raw: RawTask) {
    if let Err(e) = inner_batch(instances, &raw) {
        error!("{}", e);
        let _ = TaskDaoImpl::raw_to_error(&e, &raw);
    }
}

fn inner_batch(instances: Vec<Instance>, raw: &RawTask) -> Result<()> {
    let mut store_infos: Vec<RawTask> = Vec::new();
    let mut t_d: Vec<(TaskForStore, RawTask)> = Vec::new();
    for instance in &instances {
        let meta = MetaCacheImpl::get(&instance.meta, MG)?;
        let meta_type = meta.get_meta_type();
        let relations = RelationCacheImpl::get(&instance.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MG)?;
        let r = match meta_type {
            MetaType::Multi => RelationCacheImpl::get(&instance.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MG)?,
            _ => relations.clone(),
        };
        let mission = Mission::get_by_instance(&instance, &r, context_check, state_check);
        let task = TaskForStore::new(instance.clone(), mission, None, meta.need_cache());
        match RawTask::new(&task, &instance.get_key(), TaskType::Store as i8, &instance.meta) {
            Ok(x) => {
                store_infos.push(x.clone());
                t_d.push((task, x))
            }
            Err(e) => return Err(e)
        }
    }
    if RawTask::save_batch(&store_infos, &raw.task_id, TaskDaoImpl::insert, TaskDaoImpl::finish_task).is_ok() {
        for task in t_d {
            // if let Some(m) = &task.0.next_mission {
            //     for o in m {
            //         debug!("--store task generated: from:{},to:{}", task.0.instance.meta, o.to.meta_string());
            //     }
            // } else {
            //     debug!("----meta : {} have no missions", task.0.instance.meta);
            // }
            let _ = channel_store(task.0, task.1);
        }
    }
    Ok(())
}
