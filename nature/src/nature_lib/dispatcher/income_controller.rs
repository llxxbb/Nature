use std::convert::TryFrom;

use actix_web::web::Data;

use crate::common::*;
use crate::db::{C_M, C_R, D_M, D_R, D_T, InstanceDaoImpl, MetaCache, Mission, RawTask, RelationCache, TaskDao, TaskType};
use crate::db::flow_tool::{context_check, state_check};
use crate::domain::*;
use crate::nature_lib::dispatcher::*;
use crate::nature_lib::task::{TaskForConvert, TaskForStore};
use crate::util::*;
use crate::util::web_context::WebContext;

pub struct IncomeController {}

impl IncomeController {
    /// born an instance which is the beginning of the changes.
    pub async fn input(mut instance: Instance, context: Data<WebContext>) -> Result<u64> {
        let meta = check_and_revise(&mut instance).await?;
        let relations = C_R.get(&meta, &*D_R, &*C_M, &*D_M).await?;
        let mission = Mission::load_by_instance(&instance, &relations, context_check, state_check);
        // for o in &mission {
        //     debug!("--generate mission from:{},to:{}", &instance.meta, o.to.meta_string());
        // }
        let task = TaskForStore::new(instance.clone(), mission, None, false);
        let mut raw = task.to_raw()?;
        let num = D_T.insert(&raw).await?;
        if num > 0 {
            raw.task_id = num;
            channel_store(task, raw, context).await?;
        }
        Ok(instance.id)
    }


    /// born an instance which is the beginning of the changes.
    pub async fn self_route(instance: SelfRouteInstance, context: Data<WebContext>) -> Result<String> {
        let _ = instance.verify()?;
        // Convert a Self-Route-Instance to Normal Instance
        let mut ins = instance.to_instance();
        MetaType::check_type(&ins.path.meta, MetaType::Dynamic)?;
        let uuid = ins.revise()?.id.to_string();
        let task = TaskForStore::for_dynamic(&ins, instance.converter, None, false)?;
        let mut raw = task.to_raw()?;
        let num = D_T.insert(&raw).await?;
        if num > 0 {
            raw.task_id = num;
            channel_store(task, raw, context).await?;
        }
        Ok(uuid)
    }

    pub async fn callback(delayed: DelayedInstances, context: Data<WebContext>) -> Result<()> {
        match D_T.get(&delayed.task_id).await {
            Ok(raw) => {
                match raw {
                    None => Err(NatureError::VerifyError("task data missed, maybe it had done already.".to_string())),
                    Some(carrier) => match delayed.result {
                        ConverterReturned::Instances { ins } => {
                            let (task, last) = get_task_and_last(&carrier).await?;
                            after_converted(&task, &carrier, ins, &last, context).await
                        }
                        ConverterReturned::SelfRoute { ins: sf } => {
                            let (task, _last) = get_task_and_last(&carrier).await?;
                            received_self_route(&task, &carrier, sf)
                        }
                        ConverterReturned::Delay { num: _ } => {
                            Err(NatureError::VerifyError("callback can not process [ConverterReturned::Delay]".to_string()))
                        }
                        ConverterReturned::LogicalError { msg: err } => {
                            let err = NatureError::LogicalError(err);
                            warn!("{}", err);
                            let _ = D_T.raw_to_error(&err, &carrier).await?;
                            Ok(())
                        }
                        ConverterReturned::EnvError { msg: e } => {
                            warn!("{}", e);
                            Ok(())
                        }
                        ConverterReturned::None => {
                            let (task, last) = get_task_and_last(&carrier).await?;
                            // process_null(task.target.to.get_meta_type(), &delayed.task_id).await
                            let mut ins = Instance::default();
                            ins.path.meta = "N::1".to_string();
                            after_converted(&task, &carrier, vec![ins], &last, context).await
                        }
                    }
                }
            }
            Err(e) => Err(e)
        }
    }

    pub async fn redo_task(raw: RawTask, context: Data<WebContext>) -> Result<()> {
        // TODO check busy first
        match TaskType::try_from(raw.task_type)? {
            TaskType::Store => {
                let rtn = TaskForStore::from_raw(&raw, &*C_M, &*D_M).await?;
                debug!("--redo store task for task : {:?}", &rtn);
                channel_stored(rtn, raw, context).await;
            }
            TaskType::Convert => {
                let rtn = TaskForConvert::from_raw(&raw, InstanceDaoImpl::select_by_id, &*C_M, &*D_M).await?;
                debug!("--redo convert task: from:{}, to:{}", rtn.from.path.meta, rtn.target.to.meta_string());
                context.sender.send((rtn, raw, context.clone())).await?;
            }
            TaskType::Batch => {
                let rtn = serde_json::from_str(&raw.data)?;
                debug!("--redo batch task for task : {:?}", &rtn);
                channel_batch(rtn, raw, context).await;
            }
        }
        Ok(())
    }

    pub async fn batch(batch: Vec<Instance>, context: Data<WebContext>) -> Result<()> {
        let id = generate_id(&batch)?;
        let mut raw = RawTask::new(&batch, &id.to_string(), TaskType::Batch as i8, &batch[0].path.meta)?;
        let num = D_T.insert(&raw).await?;
        if num < 1 {
            return Ok(());
        }
        raw.task_id = num;
        let rtn = channel_batch(batch, raw, context).await;
        Ok(rtn)
    }
}

async fn get_task_and_last(task: &RawTask) -> Result<(TaskForConvert, Option<Instance>)> {
    let mut task: TaskForConvert = TaskForConvert::from_raw(task, InstanceDaoImpl::select_by_id, &*C_M, &*D_M).await?;
    let last = InstanceDaoImpl::select_last_target(&task.from, &mut task.target).await?;
    Ok((task, last))
}

async fn check_and_revise(instance: &mut Instance) -> Result<Meta> {
    let meta: Meta = C_M.get(&instance.path.meta, &*D_M).await?;    // verify meta
    // normalize meta
    instance.path.meta = meta.meta_string();
    // check previous state version
    let version = instance.path.state_version;
    if meta.is_state() && version > 1 {
        let mut kc: InsCond = instance.clone().into();
        kc.other.state_version = version - 1;
        let rtn = InstanceDaoImpl::select_by_id(kc).await?;
        if rtn.is_none() {
            return Err(NatureError::VerifyError("you can't skip state_version for instance".to_string()));
        }
    }
    let _ = instance.revise();
    Ok(meta)
}
