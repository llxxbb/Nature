use super::*;

pub struct InnerController {}

impl InnerController {
    pub fn channel_serial(task: (SerialBatchInstance, RawTask)) {
        SVC_NATURE.batch_serial_svc.do_serial_task(task.0, &task.1)
    }

    pub fn channel_parallel(task: (ParallelBatchInstance, RawTask)) {
        ParallelServiceImpl::save(task.0, task.1,
                                  TaskDaoImpl::insert, TaskDaoImpl::delete)
    }

    pub fn channel_store(store: (StoreTaskInfo, RawTask)) {
        let _ = SVC_NATURE.store_svc.do_task(&store.0, &store.1);
    }
    pub fn channel_stored(store: (StoreTaskInfo, RawTask)) {
        match ConverterInfo::generate(&store.0, &store.1,
                                      TaskDaoImpl::delete, ThingDefineCacheImpl::get, InstanceDaoImpl::get_by_id) {
            Err(err) => match err {
                NatureError::Break => return,
                e => {
                    let _ = TaskDaoImpl::raw_to_error(&e, &store.1);
                    return;
                }
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

    pub fn channel_convert(task: (ConverterInfo, RawTask)) {
        let parameter = CallOutParaSvc::gen(&task.0, task.1.task_id.clone());
        match CallerService::convert(&task.0.target, &parameter) {
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

    pub fn channel_converted(task: (ConverterInfo, Converted)) {
        if let Ok(plan) = SVC_NATURE.plan_svc.save_plan(&task.0, &task.1.converted) {
            prepare_to_store(&task.1.done_task, plan);
        }
    }

    pub fn received_instance(task: &ConverterInfo, raw: &RawTask, mut instances: &mut Vec<Instance>) -> Result<()> {
        debug!("converted {} instances for `Thing`: {:?}", instances.len(), &task.target.to);
        match Converted::gen(&task, &raw, &mut instances, ThingDefineCacheImpl::get) {
            Ok(rtn) => {
                let _ = CHANNEL_CONVERTED.sender.lock().unwrap().send((task.to_owned(), rtn));
                Ok(())
            }
            Err(err) => {
                let _ = TaskDaoImpl::raw_to_error(&err, &raw);
                Err(err)
            }
        }
    }
}


fn prepare_to_store(carrier: &RawTask, plan: PlanInfo) {
    let mut store_infos: Vec<RawTask> = Vec::new();
    let mut t_d: Vec<(StoreTaskInfo, RawTask)> = Vec::new();
    for instance in plan.plan.iter() {
        match StoreTaskInfo::gen_task(&instance, OneStepFlowCacheImpl::get, Mission::filter_relations) {
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