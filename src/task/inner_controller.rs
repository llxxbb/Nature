use super::*;

pub struct InnerController {}

impl InnerController {
    pub fn channel_store(store: (TaskForStore, RawTask)) {
        let _ = InnerController::save_instance(store.0, store.1);
    }

    pub fn save_instance(task: TaskForStore, carrier: RawTask) -> Result<()> {
        let _ = task.instance.save(InstanceDaoImpl::save)?;
        task.send(&carrier, &CHANNEL_STORED.sender.lock().unwrap());
        Ok(())
    }

    pub fn channel_stored(store: (TaskForStore, RawTask)) {
        if store.0.mission.is_none() {
            debug!("no follow data for : {}", &store.0.instance.thing.get_full_key());
            let _ = TaskDaoImpl::delete(&&store.1.task_id);
            return;
        }
        match TaskForConvert::gen_task(&store.0, ThingDefineCacheImpl::get, InstanceDaoImpl::get_by_id) {
            Err(err) => {
                let _ = TaskDaoImpl::raw_to_error(&err, &store.1);
                return;
            }
            Ok(converters) => {
                let raws: Vec<RawTask> = converters.iter().map(|x| x.1.clone()).collect();
                if RawTask::save_batch(&raws, &store.1.task_id, TaskDaoImpl::insert, TaskDaoImpl::delete).is_err() {
                    return;
                }
                debug!("will dispatch {} convert tasks for `Thing` : {:?}", converters.len(), store.0.instance.thing.get_full_key());
                for task in converters {
                    let _ = &CHANNEL_CONVERT.sender.lock().unwrap().send(task);
                }
            }
        }
    }

    pub fn channel_convert(task: (TaskForConvert, RawTask)) {
        match CallOutParaWrapper::gen_and_call_out(&task.0, task.1.task_id.clone(), &task.0.target) {
            Err(err) => match err {
                // only **Environment Error** will be retry
                NatureError::ConverterEnvironmentError(_) => (),
                // other error will drop into error
                _ => {
                    let _ = TaskDaoImpl::raw_to_error(&err, &task.1);
                }
            }
            Ok(returned) => match returned {
                ConverterReturned::Instances(mut instances) => {
                    let _ = Self::received_instance(&task.0, &task.1, &mut instances);
                }
                ConverterReturned::Delay(delay) => {
                    let _ = TaskDaoImpl::update_execute_time(&task.1.task_id, i64::from(delay));
                }
                ConverterReturned::LogicalError(ss) => {
                    let _ = TaskDaoImpl::raw_to_error(&NatureError::ConverterLogicalError(ss), &task.1);
                }
                ConverterReturned::EnvError => (),
                ConverterReturned::None => (),
            }
        };
    }

    pub fn channel_converted(task: (TaskForConvert, Converted)) {
        if let Ok(plan) = PlanInfo::save(&task.0, &task.1.converted, StorePlanDaoImpl::save, StorePlanDaoImpl::get) {
            prepare_to_store(&task.1.done_task, plan);
        }
    }

    pub fn received_instance(task: &TaskForConvert, raw: &RawTask, mut instances: &mut Vec<Instance>) -> Result<()> {
        debug!("converted {} instances for `Thing`: {:?}", instances.len(), &task.target.to);
        match Converted::gen(&task, &raw, &mut instances, ThingDefineCacheImpl::get) {
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

    pub fn channel_serial(task: (TaskForSerial, RawTask)) {
        let (task, carrier) = task;
        let finish = &task.context_for_finish.clone();
        if let Ok(si) = TaskForSerialWrapper::save(task, &ThingDefineCacheImpl::get, InstanceDaoImpl::insert) {
            match si.to_virtual_instance(finish) {
                Ok(instance) => {
                    if let Ok(si) = TaskForStore::gen_task(&instance, OneStepFlowCacheImpl::get, Mission::filter_relations) {
                        match RawTask::new(&si, &instance.thing.get_full_key(), TaskType::QueueBatch as i16) {
                            Ok(mut new) => {
                                if let Ok(_route) = new.finish_old(&carrier, TaskDaoImpl::insert, TaskDaoImpl::delete) {
                                    let _ = CHANNEL_STORED.sender.lock().unwrap().send((si, new));
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

    pub fn channel_parallel(task: (TaskForParallel, RawTask)) {
        let mut tuple: Vec<(TaskForStore, RawTask)> = Vec::new();
        for instance in task.0.instances.iter() {
            match TaskForStore::gen_task(&instance, OneStepFlowCacheImpl::get, Mission::filter_relations) {
                Ok(task) => {
                    match RawTask::save(&task, &instance.thing.get_full_key(), TaskType::Store as i16, TaskDaoImpl::insert) {
                        Ok(car) => {
                            tuple.push((task, car))
                        }
                        Err(e) => {
                            error!("{}", e);
                            return;
                        }
                    }
                }
                // any error will break the process
                _ => return
            }
        }
        if TaskDaoImpl::delete(&task.1.task_id).is_err() {
            return;
        }
        for c in tuple {
            let _ = CHANNEL_STORE.sender.lock().unwrap().send(c);
        }
    }
}


fn prepare_to_store(carrier: &RawTask, plan: PlanInfo) {
    let mut store_infos: Vec<RawTask> = Vec::new();
    let mut t_d: Vec<(TaskForStore, RawTask)> = Vec::new();
    for instance in plan.plan.iter() {
        match TaskForStore::gen_task(&instance, OneStepFlowCacheImpl::get, Mission::filter_relations) {
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
            // break process will environment error occurs.
            Err(e) => {
                error!("{}", e);
                return;
            }
        }
    }
    if RawTask::save_batch(&store_infos, &carrier.task_id, TaskDaoImpl::insert, TaskDaoImpl::delete).is_ok() {
        for task in t_d {
            let _ = CHANNEL_STORE.sender.lock().unwrap().send(task);
        }
    }
}