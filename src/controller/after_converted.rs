use nature_common::{Instance, MetaType, NatureError, Result, SelfRouteInstance};
use nature_db::{MetaCacheImpl, MetaDaoImpl, Mission, RawTask, Relation, RelationCacheImpl, RelationDaoImpl, StorePlanDaoImpl, TaskDaoImpl, TaskType};
use nature_db::flow_tool::{context_check, state_check};

use crate::actor::*;
use crate::actor::MsgForTask;
use crate::task::{Converted, PlanInfo, TaskForConvert, TaskForStore};

pub fn after_converted(task: &TaskForConvert, raw: &RawTask, instances: Vec<Instance>, last_state: &Option<Instance>) -> Result<()> {
    debug!("converted {} instances for `Meta`: {:?}", instances.len(), &task.target.to.meta_string());
    match Converted::gen(&task, &raw, instances, last_state) {
        Ok(rtn) => {
            let plan = PlanInfo::save(&task, &rtn.converted, StorePlanDaoImpl::save, StorePlanDaoImpl::get)?;
            prepare_to_store(&rtn.done_task, plan, &task.target)
        }
        Err(err) => {
            let _ = TaskDaoImpl::raw_to_error(&err, &raw);
            Err(err)
        }
    }
}

pub fn process_null(meta_type: MetaType, task_id: &[u8]) -> Result<()> {
    match meta_type {
        MetaType::Null => {
            let _ = TaskDaoImpl::delete(task_id)?;
            Ok(())
        }
        _ => Err(NatureError::VerifyError("need return [ConverterReturned::None]".to_string()))
    }
}

pub fn received_self_route(_task: &TaskForConvert, _raw: &RawTask, _instances: Vec<SelfRouteInstance>) -> Result<()> {
    // TODO unimplemented
    unimplemented!()
}

pub fn prepare_to_store(carrier: &RawTask, plan: PlanInfo, previous_mission: &Mission) -> Result<()> {
    let mut store_infos: Vec<RawTask> = Vec::new();
    let mut t_d: Vec<(TaskForStore, RawTask)> = Vec::new();
    let meta_type = previous_mission.to.get_meta_type();
    let relations = RelationCacheImpl::get(&carrier.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get)?;
    for instance in plan.plan.iter() {
        #[allow(unused_assignments)] let mut r_m: Option<Vec<Relation>> = None;
        let r = match meta_type {
            MetaType::Multi => {
                r_m = RelationCacheImpl::get(&instance.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get)?;
                &r_m
            }
            _ => &relations,
        };
        let mission = Mission::get_by_instance(instance, r, context_check, state_check);
        let task = TaskForStore::new_with_previous_mission(instance.clone(), mission, previous_mission);
        match RawTask::new(&task, &plan.to, TaskType::Store as i16) {
            Ok(x) => {
                store_infos.push(x.clone());
                t_d.push((task, x))
            }
            Err(e) => {
                error!("{}", e);
                let _ = TaskDaoImpl::raw_to_error(&e, carrier);
                return Ok(());
            }
        }
    }
    if RawTask::save_batch(&store_infos, &carrier.task_id, TaskDaoImpl::insert, TaskDaoImpl::delete).is_ok() {
        for task in t_d {
            ACT_STORE.do_send(MsgForTask(task.0, task.1));
        }
    }
    Ok(())
}
