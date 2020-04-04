use std::convert::TryFrom;

use nature_common::{ConverterReturned, DelayedInstances, generate_id, Instance, MetaType, NatureError, Result, SelfRouteInstance};
use nature_db::{INS_GETTER, InstanceDaoImpl, MCG, MetaCacheImpl, MG, Mission, RawTask, RelationCacheImpl, RelationDaoImpl, TaskDaoImpl, TaskType};
use nature_db::flow_tool::{context_check, state_check};

use crate::controller::*;
use crate::task::{TaskForConvert, TaskForStore};

pub struct IncomeController {}

impl IncomeController {
    /// born an instance which is the beginning of the changes.
    pub async fn input(mut instance: Instance) -> Result<u128> {
        let _ = instance.check_and_revise(MetaCacheImpl::get, MG)?;
        let relations = RelationCacheImpl::get(&instance.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MG)?;
        let mission = Mission::get_by_instance(&instance, &relations, context_check, state_check);
        let task = TaskForStore::new(instance.clone(), mission, None, false);
        let raw = task.to_raw()?;
        TaskDaoImpl::insert(&raw)?;
        channel_store(task, raw).await?;
        Ok(instance.id)
    }

    /// born an instance which is the beginning of the changes.
    pub async fn self_route(instance: SelfRouteInstance) -> Result<u128> {
        let _ = instance.verify()?;
        // Convert a Self-Route-Instance to Normal Instance
        let mut ins = instance.to_instance();
        MetaType::check_type(&ins.meta, MetaType::Dynamic)?;
        let uuid = ins.revise()?.id;
        let task = TaskForStore::for_dynamic(&ins, instance.converter, None, false)?;
        let raw = task.to_raw()?;
        let _ = TaskDaoImpl::insert(&raw)?;
        channel_store(task, raw).await?;
        Ok(uuid)
    }

    pub async fn callback(delayed: DelayedInstances) -> Result<()> {
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
                        ConverterReturned::EnvError(e) => {
                            warn!("{}", e);
                            Ok(())
                        }
                        ConverterReturned::Delay(_) => {
                            Err(NatureError::VerifyError("callback can not process [ConverterReturned::Delay]".to_string()))
                        }
                        ConverterReturned::Instances(ins) => {
                            let (task, last) = get_task_and_last(&carrier.data)?;
                            after_converted(&task, &carrier, ins, &last).await
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

    pub async fn redo_task(raw: RawTask) -> Result<()> {
        // TODO check busy first
        match TaskType::try_from(raw.task_type)? {
            TaskType::Store => {
                let rtn = TaskForStore::from_raw(&raw.data, INS_GETTER, MCG, MG)?;
                debug!("--redo store task for task : {:?}", &rtn);
                channel_stored(rtn, raw).await;
            }
            TaskType::Convert => {
                let rtn = TaskForConvert::from_raw(&raw.data, INS_GETTER, MCG, MG)?;
                debug!("--redo convert task: from:{}, to:{}", rtn.from.meta, rtn.target.to.meta_string());
                channel_convert(rtn, raw).await;
            }
            TaskType::Batch => {
                let rtn = serde_json::from_str(&raw.data)?;
                debug!("--redo batch task for task : {:?}", &rtn);
                channel_batch(rtn, raw);
            }
        }
        Ok(())
    }

    pub async fn batch(batch: Vec<Instance>) -> Result<()> {
        let id = generate_id(&batch)?;
        let raw = RawTask::new(&batch, &id.to_string(), TaskType::Batch as i8, &batch[0].meta)?;
        let _ = TaskDaoImpl::insert(&raw)?;
        let rtn = channel_batch(batch, raw);
        Ok(rtn)
    }
}

fn get_task_and_last(task: &str) -> Result<(TaskForConvert, Option<Instance>)> {
    let task: TaskForConvert = TaskForConvert::from_raw(task, INS_GETTER, MCG, MG)?;
    let last = match task.target.to.is_state() {
        true => task.from.get_last_taget(&task.target.to.meta_string(), InstanceDaoImpl::get_last_state)?,
        false => None
    };
    Ok((task, last))
}