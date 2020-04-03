use nature_common::{Instance, MetaType, NatureError, Result, SelfRouteInstance};
use nature_db::{MetaCacheImpl, MG, Mission, RawTask, RelationCacheImpl, RelationDaoImpl, TaskDaoImpl, TaskType};
use nature_db::flow_tool::{context_check, state_check};

use crate::controller::{channel_batch, channel_store};
use crate::task::{Converted, TaskForConvert, TaskForStore};

pub async fn after_converted(task: &TaskForConvert, convert_task: &RawTask, instances: Vec<Instance>, last_state: &Option<Instance>) -> Result<()> {
    debug!("converted {} instances for `Meta`: {:?}, from {}", instances.len(), &task.target.to.meta_string(), task.from.get_key());
    match Converted::gen(&task, &convert_task, instances, last_state) {
        Ok(rtn) => match rtn.converted.len() {
            0 => match TaskDaoImpl::finish_task(&convert_task.task_id) {
                Ok(_) => Ok(()),
                Err(e) => Err(e)
            },
            1 => save_one(rtn, &task.target).await,
            _ => save_batch(rtn),
        }
        Err(err) => {
            let _ = TaskDaoImpl::raw_to_error(&err, &convert_task);
            Err(err)
        }
    }
}

pub async fn save_one(converted: Converted, previous_mission: &Mission) -> Result<()> {
    let instance = &converted.converted[0];
    let relations = RelationCacheImpl::get(&instance.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MG)?;
    let mission = Mission::get_by_instance(instance, &relations, context_check, state_check);
    let meta = MetaCacheImpl::get(&instance.meta, MG)?;
    let task = TaskForStore::new(instance.clone(), mission, Some(previous_mission.clone()), meta.need_cache());
    let rtn = channel_store(task, converted.done_task).await?;
    Ok(rtn)
}

pub fn save_batch(converted: Converted) -> Result<()> {
    let raw = RawTask::new(&converted.converted, &converted.done_task.task_key, TaskType::Batch as i8, "")?;
    let _ = TaskDaoImpl::insert(&raw)?;
    let _ = TaskDaoImpl::finish_task(&converted.done_task.task_id)?;
    let rtn = channel_batch(converted.converted, raw);
    Ok(rtn)
}

pub fn process_null(meta_type: MetaType, task_id: &[u8]) -> Result<()> {
    match meta_type {
        MetaType::Null => {
            let _ = TaskDaoImpl::finish_task(task_id)?;
            Ok(())
        }
        _ => Err(NatureError::VerifyError("need return [ConverterReturned::None]".to_string()))
    }
}

pub fn received_self_route(_task: &TaskForConvert, _raw: &RawTask, _instances: Vec<SelfRouteInstance>) -> Result<()> {
    // TODO unimplemented
    unimplemented!()
}
