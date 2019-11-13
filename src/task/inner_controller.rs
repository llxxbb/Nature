use serde::export::From;

use nature_common::{CONTEXT_TARGET_INSTANCE_ID, ConverterReturned, Instance, NatureError, ParaForIDAndFrom, ParaForQueryByID, Protocol, Result, SelfRouteInstance, TaskForSerial};
use nature_db::{InstanceDaoImpl, MetaCacheImpl, MetaDaoImpl, Mission, RawTask, RelationCacheImpl, RelationDaoImpl, StorePlanDaoImpl, TaskDaoImpl, TaskType};

use crate::actor::*;
use crate::task::{Converted, ConverterParameterWrapper, PlanInfo, TaskForConvert, TaskForSerialWrapper, TaskForStore};

pub struct InnerController {}

/// The core process of the Nature
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
            Err(NatureError::DaoDuplicated(err)) => duplicated_instance(&task, &carrier, err),
            Err(e) => Err(e)
        }
    }

    pub fn channel_stored(task: TaskForStore, raw: RawTask) {
        if task.next_mission.is_none() {
            let _ = TaskDaoImpl::delete(&&raw.task_id);
            return;
        }
        match TaskForConvert::gen_task(&task) {
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
        let protocol = task.target.executor.protocol.clone();
        let mut from_instance = task.from.clone();
        // -----begin this logic can't move to place where after converted, because it might not get the last state and cause state conflict
        if protocol == Protocol::Auto {
            let id = from_instance.id.to_string();
            from_instance.context.insert(CONTEXT_TARGET_INSTANCE_ID.to_string(), id);
        }
        // -----end
        let last = match task.target.to.is_state() {
            true => match from_instance.get_last_taget(&task.target.to.meta_string(), InstanceDaoImpl::get_by_id) {
                Err(_) => { return; }
                Ok(last) => last
            }
            false => None
        };
        if Protocol::Auto == protocol {
            let _ = Self::after_converted(&task, &raw, vec![Instance::default()], &last);
            return;
        }
        // init master
        let meta = match MetaCacheImpl::get(&task.from.meta, MetaDaoImpl::get) {
            Ok(m) => m,
            Err(e) => {
                let _ = TaskDaoImpl::raw_to_error(&e, &raw);
                return;
            }
        };
        let master = match task.from.get_master(&meta, InstanceDaoImpl::get_by_id) {
            Ok(m) => m,
            Err(e) => {
                let _ = TaskDaoImpl::raw_to_error(&e, &raw);
                return;
            }
        };
        match ConverterParameterWrapper::gen_and_call_out(&task, raw.task_id.clone(), &task.target, &last, master) {
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
                    let _ = Self::after_converted(&task, &raw, instances, &last);
                }
                ConverterReturned::SelfRoute(ins) => {
                    let _ = Self::received_self_route(&task, &raw, ins);
                }
                ConverterReturned::Delay(delay) => {
                    let _ = TaskDaoImpl::update_execute_time(&raw.task_id, i64::from(delay), &last);
                }
                ConverterReturned::LogicalError(ss) => {
                    let _ = TaskDaoImpl::raw_to_error(&NatureError::ConverterLogicalError(ss), &raw);
                }
                ConverterReturned::EnvError => (),
                ConverterReturned::None => (),
            }
        };
    }

    pub fn after_converted(task: &TaskForConvert, raw: &RawTask, instances: Vec<Instance>, last_state: &Option<Instance>) -> Result<()> {
        debug!("converted {} instances for `Meta`: {:?}", instances.len(), &task.target.to.get_full_key());
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

    pub fn received_self_route(_task: &TaskForConvert, _raw: &RawTask, _instances: Vec<SelfRouteInstance>) -> Result<()> {
        // TODO unimplemented
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

fn duplicated_instance(task: &TaskForStore, carrier: &RawTask, err: String) -> Result<()> {
    // none state-meta process
    if task.instance.state_version == 0 {
        warn!("Instance duplicated for id : {}, of `Meta` : {}, will delete it's task!", task.instance.id, &task.instance.meta);
        // Don't worry about the previous task would deleted while in processing!, the old task will be continue.
        let _ = TaskDaoImpl::delete(&&carrier.task_id);
        return Err(NatureError::DaoDuplicated(err));
    }

    let ins_from = task.instance.from.clone().unwrap();
    let para = ParaForIDAndFrom {
        id: task.instance.id,
        meta: task.instance.meta.clone(),
        from_id: ins_from.id,
        from_meta: ins_from.meta.clone(),
        from_state_version: ins_from.state_version,
    };
    let old = InstanceDaoImpl::get_by_from(&para)?;
    if let Some(ins) = old {
        // same frominstance
        warn!("same from-instance for meta: [{}] on version : {}", &task.instance.meta, task.instance.state_version);
        let task = TaskForStore::new(ins, Some(vec![task.previous_mission.clone().unwrap()]));
        ACT_STORED.try_send(MsgForTask(task, carrier.clone()))?;
        return Ok(());
    } else {
        warn!("conflict for state-meta: [{}] on version : {}", &task.instance.meta, task.instance.state_version);
        let _ = StorePlanDaoImpl::delete(&ins_from.get_upstream(), &task.instance.meta)?;
        let ins = InstanceDaoImpl::get_by_id(&ParaForQueryByID::new(ins_from.id, &ins_from.meta))?;
        match ins {
            Some(ins) => {
                let task = TaskForStore::new(ins, Some(vec![task.previous_mission.clone().unwrap()]));
                ACT_STORED.try_send(MsgForTask(task, carrier.clone()))?;
                return Ok(());
            }
            None => {
                let error = NatureError::VerifyError("from-instance does not found".to_string());
                let _ = TaskDaoImpl::raw_to_error(&error, &carrier);
                return Err(error);
            }
        }
    }
}

fn prepare_to_store(carrier: &RawTask, plan: PlanInfo, previous_mission: &Mission) -> Result<()> {
    let mut store_infos: Vec<RawTask> = Vec::new();
    let mut t_d: Vec<(TaskForStore, RawTask)> = Vec::new();
    let relations = RelationCacheImpl::get(&carrier.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get)?;
    for instance in plan.plan.iter() {
        let mission = Mission::get_by_instance(instance, &relations);
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

fn inner_serial(task: &MsgForTask<TaskForSerial>) -> Result<()> {
    let (task, carrier) = (&task.0, &task.1);
    let finish = &task.context_for_finish.clone();
    match TaskForSerialWrapper::save(task, InstanceDaoImpl::insert) {
        Ok(sf) => {
            let ins = sf.to_virtual_instance(finish)?;
            match RelationCacheImpl::get(&ins.meta, RelationDaoImpl::get_relations, MetaCacheImpl::get, MetaDaoImpl::get) {
                Ok(relations) => {
                    let mission = Mission::get_by_instance(&ins, &relations);
                    let store_task = TaskForStore::new(ins.clone(), mission);
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
                let task = TaskForStore::new(instance.clone(), mission);
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
