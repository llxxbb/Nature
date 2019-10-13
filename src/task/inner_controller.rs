use serde::export::From;

use nature_common::{ConverterReturned, Instance, NatureError, Protocol, Result, SelfRouteInstance, TaskForSerial};
use nature_db::{InstanceDaoImpl, MetaCacheImpl, MetaDaoImpl, Mission, RawTask, RelationCacheImpl, RelationDaoImpl, StorePlanDaoImpl, TaskDaoImpl, TaskType};

use crate::actor::*;
use crate::task::{Converted, ConverterParameterWrapper, PlanInfo, TaskForConvert, TaskForSerialWrapper, TaskForStore};

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
                warn!("Instance duplicated for id : {}, of `Meta` : {}, will delete it's task!", task.instance.id, &task.instance.meta);
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
        match TaskForConvert::gen_task(&task, InstanceDaoImpl::get_by_id) {
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
        if Protocol::Auto == task.target.executor.protocol {
            let _ = Self::received_instance(&task, &raw, vec![Instance::default()]);
            return;
        }
        match ConverterParameterWrapper::gen_and_call_out(&task, raw.task_id.clone(), &task.target) {
            Err(err) => match err {
                // only **Environment Error** will be retry
                NatureError::EnvironmentError(_) => (),
                // other error will drop into error
                _ => {
                    let _ = TaskDaoImpl::raw_to_error(&err, &raw);
                }
            }
            Ok(returned) => match returned {
                ConverterReturned::Instances(instances) => {
                    let _ = Self::received_instance(&task, &raw, instances);
                }
                ConverterReturned::SelfRoute(ins) => {
                    let _ = Self::received_self_route(&task, &raw, ins);
                }
                ConverterReturned::Mixed((ins, sf)) => {
                    let _ = Self::received_instance(&task, &raw, ins);
                    let _ = Self::received_self_route(&task, &raw, sf);
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
        match Converted::gen(&task, &raw, instances) {
            Ok(rtn) => {
                let plan = PlanInfo::save(&task, &rtn.converted, StorePlanDaoImpl::save, StorePlanDaoImpl::get)?;
                prepare_to_store(&rtn.done_task, plan)
            }
            Err(err) => {
                let _ = TaskDaoImpl::raw_to_error(&err, &raw);
                Err(err)
            }
        }
    }

    pub fn received_self_route(_task: &TaskForConvert, _raw: &RawTask, _instances: Vec<SelfRouteInstance>) -> Result<()> {
        // TODO
        unimplemented!()
    }

    pub fn channel_serial(task: MsgForTask<TaskForSerial>) {
        if let Err(e) = inner_serial(&task) {
            let _ = TaskDaoImpl::raw_to_error(&e, &task.1);
        }
    }


    pub fn channel_parallel(task: MsgForTask<Vec<Instance>>) {
        if let Err(e) = inner_parallel(&task) {
            let _ = TaskDaoImpl::raw_to_error(&e, &task.1);
        }
    }
}

fn prepare_to_store(carrier: &RawTask, plan: PlanInfo) -> Result<()> {
    let mut store_infos: Vec<RawTask> = Vec::new();
    let mut t_d: Vec<(TaskForStore, RawTask)> = Vec::new();
    let relations = RelationCacheImpl::get(&carrier.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get)?;
    for instance in plan.plan.iter() {
        let mission = Mission::get_by_instance(instance, &relations);
        let task = TaskForStore { instance: instance.clone(), mission };
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

fn inner_serial(task: &MsgForTask<TaskForSerial>) -> Result<()> {
    let (task, carrier) = (&task.0, &task.1);
    let finish = &task.context_for_finish.clone();
    match TaskForSerialWrapper::save(task, InstanceDaoImpl::insert) {
        Ok(sf) => {
            let ins = sf.to_virtual_instance(finish)?;
            match RelationCacheImpl::get(&ins.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get) {
                Ok(relations) => {
                    let mission = Mission::get_by_instance(&ins, &relations);
                    let store_task = TaskForStore { instance: ins.clone(), mission };
                    let mut raw = RawTask::new(&store_task, &ins.meta, TaskType::QueueBatch as i16)?;
                    if let Ok(_route) = raw.finish_old(&carrier, TaskDaoImpl::insert, TaskDaoImpl::delete) {
                        let _ = ACT_STORED.try_send(MsgForTask(store_task, raw));
                    }
                    Ok(())
                }
                Err(NatureError::EnvironmentError(_)) => Ok(()),
                Err(e) => Err(e)
            }
        }
        Err(NatureError::EnvironmentError(_)) => Ok(()),
        Err(e) => Err(e)
    }
}

fn inner_parallel(task: &MsgForTask<Vec<Instance>>) -> Result<()> {
    let mut tuple: Vec<(TaskForStore, RawTask)> = Vec::new();
    match RelationCacheImpl::get(&task.0[0].meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get) {
        Ok(relations) => {
            for instance in task.0.iter() {
                let mission = Mission::get_by_instance(&instance, &relations);
                let task = TaskForStore { instance: instance.clone(), mission };
                let raw = RawTask::new(&task, &instance.meta, TaskType::Store as i16)?;
                match TaskDaoImpl::insert(&raw) {
                    Ok(_) => tuple.push((task, raw)),
                    Err(NatureError::EnvironmentError(_)) => return Ok(()),
                    Err(e) => return Err(e)
                }
            }
            for c in tuple {
                ACT_STORE.do_send(MsgForTask(c.0, c.1));
            }
            let _ = TaskDaoImpl::delete(&task.1.task_id);
            Ok(())
        }
        Err(NatureError::EnvironmentError(_)) => Ok(()),
        Err(e) => Err(e)
    }
}
