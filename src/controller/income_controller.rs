use std::convert::TryFrom;

use nature_common::{ConverterReturned, DelayedInstances, Instance, MetaType, NatureError, Result, SelfRouteInstance};
use nature_db::{InstanceDaoImpl, MetaCacheImpl, MetaDaoImpl, Mission, RawTask, RelationCacheImpl, RelationDaoImpl, TaskDaoImpl, TaskType};

use crate::actor::*;
use crate::controller::*;
use crate::task::{TaskForConvert, TaskForStore};
use nature_db::flow_tool::{context_check, state_check};

pub struct IncomeController {}

impl IncomeController {
    /// born an instance which is the beginning of the changes.
    pub fn input(mut instance: Instance) -> Result<u128> {
        let _ = instance.check_and_revise(MetaCacheImpl::get, MetaDaoImpl::get)?;
        let relations = RelationCacheImpl::get(&instance.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get)?;
        let mission = Mission::get_by_instance(&instance, &relations, context_check, state_check);
        let task = TaskForStore::new(instance.clone(), mission);
        let raw = RawTask::new(&task, &instance.meta, TaskType::Store as i16)?;
        TaskDaoImpl::insert(&raw)?;
        save_instance(task, raw)?;
        Ok(instance.id)
    }

    /// born an instance which is the beginning of the changes.
    pub fn self_route(instance: SelfRouteInstance) -> Result<u128> {
        let _ = instance.verify()?;
        // Convert a Self-Route-Instance to Normal Instance
        let mut ins = instance.to_instance();
        MetaType::check_type(&ins.meta, MetaType::Dynamic)?;
        let uuid = ins.revise()?.id;
        let task = TaskForStore::for_dynamic(&ins, instance.converter)?;
        let raw = RawTask::new(&task, &ins.meta, TaskType::Store as i16)?;
        let _ = TaskDaoImpl::insert(&raw)?;
        save_instance(task, raw)?;
        Ok(uuid)
    }

    pub fn callback(delayed: DelayedInstances) -> Result<()> {
        match TaskDaoImpl::get(&delayed.task_id) {
            Ok(raw) => {
                match raw {
                    None => Err(NatureError::VerifyError("task data missed, maybe it had done already.".to_string())),
                    Some(carrier) => match delayed.result {
                        ConverterReturned::LogicalError(err) => {
                            let err = NatureError::LogicalError(err);
                            let _ = TaskDaoImpl::raw_to_error(&err, &carrier)?;
                            Ok(())
                        }
                        ConverterReturned::EnvError => {
                            Ok(())
                        }
                        ConverterReturned::Delay(_) => {
                            Err(NatureError::VerifyError("callback can not process [ConverterReturned::Delay]".to_string()))
                        }
                        ConverterReturned::Instances(ins) => {
                            let (task, last) = get_task_and_last(&carrier.data)?;
                            after_converted(&task, &carrier, ins, &last)
                        }
                        ConverterReturned::SelfRoute(sf) => {
                            let (task, _last) = get_task_and_last(&carrier.data)?;
                            received_self_route(&task, &carrier, sf)
                        }
                        ConverterReturned::None => {
                            let (task, _last) = get_task_and_last(&carrier.data)?;
                            process_null(task.target.to.get_meta_type(), &delayed.task_id)
                        }
                    }
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn redo_task(raw: RawTask) -> Result<()> {
        // TODO check busy first
        match TaskType::try_from(raw.data_type)? {
            TaskType::Store => {
                let rtn = serde_json::from_str(&raw.data)?;
                ACT_STORED.do_send(MsgForTask(rtn, raw));
            }
            TaskType::Convert => {
                let rtn = serde_json::from_str(&raw.data)?;
                ACT_CONVERT.do_send(MsgForTask(rtn, raw));
            }
            TaskType::Batch => {
                let rtn = serde_json::from_str(&raw.data)?;
                ACT_BATCH.do_send(MsgForTask(rtn, raw));
            }
        }
        Ok(())
    }

    pub fn batch(batch: Vec<Instance>) -> Result<()> {
        let _ = Instance::meta_must_same(&batch)?;
        let raw = RawTask::new(&batch, &batch[0].meta, TaskType::Batch as i16)?;
        let _ = TaskDaoImpl::insert(&raw)?;
        let _ = ACT_BATCH.try_send(MsgForTask(batch, raw));
        Ok(())
    }
}

fn get_task_and_last(task: &str) -> Result<(TaskForConvert, Option<Instance>)> {
    let task: TaskForConvert = serde_json::from_str(task)?;
    let last = match task.target.to.is_state() {
        true => task.from.get_last_taget(&task.target.to.meta_string(), InstanceDaoImpl::get_by_id)?,
        false => None
    };
    Ok((task, last))
}