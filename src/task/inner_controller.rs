use serde::export::From;

use nature_common::{ConverterReturned, Instance, NatureError, Result, TaskForParallel, TaskForSerial};
use nature_db::{InstanceDaoImpl, MetaCacheImpl, MetaDaoImpl, Mission, OneStepFlowCacheImpl, OneStepFlowDaoImpl, RawTask, StorePlanDaoImpl, TaskDaoImpl, TaskType};

use crate::actor::*;
use crate::task::{CallOutParaWrapper, Converted, PlanInfo, TaskForConvert, TaskForSerialWrapper, TaskForStore};

pub struct InnerController {}

impl InnerController {
    pub fn channel_store(store: (TaskForStore, RawTask)) {
        let _ = InnerController::save_instance(store.0, store.1);
    }

    pub fn save_instance(task: TaskForStore, carrier: RawTask) -> Result<()> {
        match InstanceDaoImpl::insert(&task.instance) {
            Ok(_) => {
                ACT_STORED.try_send(MsgForTask(task, carrier))?;
                Ok(())
            }
            Err(NatureError::DaoDuplicated(err)) => {
                warn!("Instance duplicated for id : {}, of `Meta` : {:?}, will delete it's task!", task.instance.id, &task.instance.meta.get_full_key());
                // Don't worry about the previous task would deleted while in processing!
                // the task will be duplicated too or an new one for same instance.
                let _ = TaskDaoImpl::delete(&&carrier.task_id);
                Err(NatureError::DaoDuplicated(err))
            }
            Err(e) => Err(e)
        }
    }

    pub fn channel_stored(task: TaskForStore, raw: RawTask) {
        if task.mission.is_none() {
            let _ = TaskDaoImpl::delete(&&raw.task_id);
            return;
        }
        match TaskForConvert::gen_task(&task, MetaCacheImpl::get, MetaDaoImpl::get, InstanceDaoImpl::get_by_id) {
            Ok(converters) => {
                let raws: Vec<RawTask> = converters.iter().map(|x| x.1.clone()).collect();
                if RawTask::save_batch(&raws, &raw.task_id, TaskDaoImpl::insert, TaskDaoImpl::delete).is_err() {
                    return;
                }
                for t in converters {
                    let _ = ACT_CONVERT.try_send(MsgForTask(t.0, t.1));
                }
            }
            Err(err) => {
                let _ = TaskDaoImpl::raw_to_error(&err, &raw);
                return;
            }
        }
    }

    pub fn channel_convert(task: TaskForConvert, raw: RawTask) {
        match CallOutParaWrapper::gen_and_call_out(&task, raw.task_id.clone(), &task.target) {
            Err(err) => match err {
                // only **Environment Error** will be retry
                NatureError::ConverterEnvironmentError(_) => (),
                // other error will drop into error
                _ => {
                    let _ = TaskDaoImpl::raw_to_error(&err, &raw);
                }
            }
            Ok(returned) => match returned {
                ConverterReturned::Instances(instances) => {
                    let _ = Self::received_instance(&task, &raw, instances);
                }
                ConverterReturned::Delay(delay) => {
                    let _ = TaskDaoImpl::update_execute_time(&raw.task_id, i64::from(delay));
                }
                ConverterReturned::LogicalError(ss) => {
                    let _ = TaskDaoImpl::raw_to_error(&NatureError::ConverterLogicalError(ss), &raw);
                }
                ConverterReturned::EnvError => (),
                ConverterReturned::None => (),
            }
        };
    }

    pub fn received_instance(task: &TaskForConvert, raw: &RawTask, instances: Vec<Instance>) -> Result<()> {
        debug!("converted {} instances for `Meta`: {:?}", instances.len(), &task.target.to.get_full_key());
        match Converted::gen(&task, &raw, instances, MetaCacheImpl::get, MetaDaoImpl::get) {
            Ok(rtn) => {
                let plan = PlanInfo::save(&task, &rtn.converted, StorePlanDaoImpl::save, StorePlanDaoImpl::get)?;
                Ok(prepare_to_store(&rtn.done_task, plan))
            }
            Err(err) => {
                let _ = TaskDaoImpl::raw_to_error(&err, &raw);
                Err(err)
            }
        }
    }

    pub fn channel_serial(task: MsgForTask<TaskForSerial>) {
        let (task, carrier) = (task.0, task.1);
        let finish = &task.context_for_finish.clone();
        if let Ok(si) = TaskForSerialWrapper::save(task, MetaCacheImpl::get, MetaDaoImpl::get, InstanceDaoImpl::insert) {
            match si.to_virtual_instance(finish) {
                Ok(instance) => {
                    if let Ok(si) = TaskForStore::gen_task(&instance, OneStepFlowCacheImpl::get, OneStepFlowDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get, Mission::filter_relations) {
                        match RawTask::new(&si, &instance.meta.get_full_key(), TaskType::QueueBatch as i16) {
                            Ok(mut new) => {
                                if let Ok(_route) = new.finish_old(&carrier, TaskDaoImpl::insert, TaskDaoImpl::delete) {
                                    let _ = ACT_STORED.try_send(MsgForTask(si, new));
                                }
                            }
                            Err(err) => {
                                let _ = TaskDaoImpl::raw_to_error(&err, &carrier);
                            }
                        }
                    }
                }
                Err(err) => {
                    let _ = TaskDaoImpl::raw_to_error(&err, &carrier);
                }
            };
        }
    }

    pub fn channel_parallel(task: MsgForTask<TaskForParallel>) {
        let mut tuple: Vec<(TaskForStore, RawTask)> = Vec::new();
        let mut err: Option<NatureError> = None;
        for instance in task.0.instances.iter() {
            match TaskForStore::gen_task(&instance, OneStepFlowCacheImpl::get, OneStepFlowDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get, Mission::filter_relations) {
                Ok(task) => {
                    match RawTask::new(&task, &instance.meta.get_full_key(), TaskType::Store as i16) {
                        Ok(raw) => {
                            match TaskDaoImpl::insert(&raw) {
                                Ok(_) => {
                                    tuple.push((task, raw))
                                }
                                Err(e) => {
                                    err = Some(e);
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            err = Some(e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    err = Some(e);
                    break;
                }
            }
        }
        match err {
            None => {
                for c in tuple {
                    ACT_STORE.do_send(MsgForTask(c.0, c.1));
                }
                let _ = TaskDaoImpl::delete(&task.1.task_id);
            }
            Some(NatureError::DaoLogicalError(s)) => {
                let _ = TaskDaoImpl::raw_to_error(&NatureError::DaoLogicalError(s), &task.1);
            }
            Some(_) => ()
        }
    }
}


fn prepare_to_store(carrier: &RawTask, plan: PlanInfo) {
    let mut store_infos: Vec<RawTask> = Vec::new();
    let mut t_d: Vec<(TaskForStore, RawTask)> = Vec::new();
    for instance in plan.plan.iter() {
        match TaskForStore::gen_task(&instance, OneStepFlowCacheImpl::get, OneStepFlowDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get, Mission::filter_relations) {
            Ok(task) => {
                match RawTask::new(&task, &plan.to.get_full_key(), TaskType::Store as i16) {
                    Ok(x) => {
                        store_infos.push(x.clone());
                        t_d.push((task, x))
                    }
                    Err(e) => {
                        error!("{}", e);
                        let _ = TaskDaoImpl::raw_to_error(&e, carrier);
                        return;
                    }
                }
            }
            // break process when environment error occurs.
            Err(e) => {
                warn!("{}", e);
                return;
            }
        }
    }
    if RawTask::save_batch(&store_infos, &carrier.task_id, TaskDaoImpl::insert, TaskDaoImpl::delete).is_ok() {
        for task in t_d {
            ACT_STORE.do_send(MsgForTask(task.0, task.1));
        }
    }
}