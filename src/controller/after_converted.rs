use nature_common::{Instance, MetaType, NatureError, Result, SelfRouteInstance};
use nature_db::{C_M, C_R, D_M, D_R, D_T, MetaCache, Mission, RawTask, RelationCache, TaskDao, TaskType};
use nature_db::flow_tool::{context_check, state_check};

use crate::controller::{channel_batch, channel_store};
use crate::system::SWITCH_SAVE_DIRECTLY_FOR_ONE;
use crate::task::{Converted, gen_loop_mission, TaskForConvert, TaskForStore};

pub async fn after_converted(task: &TaskForConvert, convert_task: &RawTask, instances: Vec<Instance>, last_state: &Option<Instance>) -> Result<()> {
    // debug!("executor returned {} instances for `Meta`: {:?}, from {}", instances.len(), &task.target.to.meta_string(), task.from.get_key());
    match Converted::gen(&task, &convert_task, instances, last_state) {
        Ok(rtn) => {
            match rtn.converted.len() {
                0 => match D_T.finish_task(&convert_task.task_id).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("finish task occur error: {}", e);
                        Err(e)
                    }
                },
                1 => {
                    if loop_check(task, &rtn.converted[0], convert_task).await { return Ok(()); }
                    match *SWITCH_SAVE_DIRECTLY_FOR_ONE {
                        true => save_one(rtn, &task.target).await,
                        false => save_batch(rtn).await
                    }
                }
                _ => save_batch(rtn).await
            }
        }
        Err(err) => {
            warn!("pre-process returned instance error:{}, task would be moved to error table", err);
            let _ = D_T.raw_to_error(&err, &convert_task).await;
            Err(err)
        }
    }
}

async fn loop_check(task: &TaskForConvert, ins: &Instance, raw: &RawTask) -> bool {
    if ins.state_version > 0 && ins.state_version == task.conflict_version {
        warn!("looping for conflict: {}, task would be moved to error table", ins.get_key());
        let err = NatureError::LogicalError("conflict looping".to_string());
        let _ = D_T.raw_to_error(&err, &raw).await;
        true
    } else {
        false
    }
}


async fn save_one(converted: Converted, previous_mission: &Mission) -> Result<()> {
    let instance = &converted.converted[0];
    let mission = match previous_mission.to.get_meta_type() {
        MetaType::Loop => {
            gen_loop_mission(instance, &*C_M, &*D_M).await?
        }
        _ => {
            let relations = C_R.get(&instance.meta, &*D_R, &*C_M, &*D_M).await?;
            Mission::get_by_instance(instance, &relations, context_check, state_check)
        }
    };
    let meta = C_M.get(&instance.meta, &*D_M).await?;
    let task = TaskForStore::new(instance.clone(), mission, Some(previous_mission.clone()), meta.need_cache());
    let rtn = channel_store(task, converted.done_task).await?;
    Ok(rtn)
}

async fn save_batch(converted: Converted) -> Result<()> {
    let raw = RawTask::new(&converted.converted, &converted.done_task.task_key, TaskType::Batch as i8, "")?;
    let _ = D_T.insert(&raw).await?;
    let _ = D_T.finish_task(&converted.done_task.task_id).await?;
    let rtn = channel_batch(converted.converted, raw).await;
    Ok(rtn)
}

pub async fn process_null(meta_type: MetaType, task_id: &str) -> Result<()> {
    match meta_type {
        MetaType::Null => {
            let _ = D_T.finish_task(task_id).await?;
            Ok(())
        }
        _ => Err(NatureError::VerifyError("need return [ConverterReturned::None]".to_string()))
    }
}

pub fn received_self_route(_task: &TaskForConvert, _raw: &RawTask, _instances: Vec<SelfRouteInstance>) -> Result<()> {
    // TODO unimplemented
    unimplemented!()
}
