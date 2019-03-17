use super::*;

pub struct InnerController {}

impl InnerController {
    pub fn channel_serial(task: (SerialBatchInstance, RawTask)) {
        SVC_NATURE.batch_serial_svc.do_serial_task(task.0, &task.1)
    }

    pub fn channel_parallel(task: (ParallelBatchInstance, RawTask)) {
        SVC_NATURE.batch_parallel_svc.do_parallel_task(task.0, task.1)
    }

    pub fn channel_store(store: (StoreTaskInfo, RawTask)) {
        let _ = SVC_NATURE.store_svc.do_task(&store.0, &store.1);
    }
    pub fn channel_stored(store: (StoreTaskInfo, RawTask)) {
        debug!("------------------channel_stored------------------------");
        let biz = store.0.instance.thing.key.clone();
        if store.0.mission.is_none() {
            debug!("no follow data for : {}", biz);
            let _ = SVC_NATURE.task_dao.delete(&&store.1.task_id);
            return;
        }
        let converters = match SVC_NATURE.converter_svc.generate_converter_info(&store.0) {
            Ok(new) => new,
            Err(err) => match err {
                NatureError::DaoEnvironmentError(_) => return,
                _ => {
                    debug!("get `one step info` error for : {}", biz);
                    let _ = SVC_NATURE.task_dao.raw_to_error(&err, &store.1);
                    return;
                }
            }
        };
        let raws: Vec<RawTask> = converters.iter().map(|x| x.1.clone()).collect();
        if SVC_NATURE.task_svc.create_batch_and_finish_carrier(&raws, &store.1.task_id).is_ok() {
            debug!("will dispatch {} convert tasks for `Thing` : {:?}", converters.len(), biz);
            for task in converters {
                let _ = CHANNEL_CONVERT.sender.lock().unwrap().send(task);
            }
        };
    }

    pub fn channel_convert(task: (ConverterInfo, RawTask)) {
        SVC_NATURE.converter_svc.convert(&task.0, &task.1);
    }
    pub fn channel_converted(task: (ConverterInfo, Converted)) {
        if let Ok(plan) = SVC_NATURE.plan_svc.save_plan(&task.0, &task.1.converted) {
            Self::prepare_to_store(&task.1.done_task, plan);
        }
    }
    fn prepare_to_store(carrier: &RawTask, plan: PlanInfo) {
        let mut store_infos: Vec<RawTask> = Vec::new();
        let mut t_d: Vec<(StoreTaskInfo, RawTask)> = Vec::new();
        for instance in plan.plan.iter() {
            match SVC_NATURE.store_svc.generate_store_task(instance) {
                Ok(task) => {
                    match RawTask::new(&task, &plan.to.key, TaskType::Store as i16) {
                        Ok(x) => {
                            store_infos.push(x.clone());
                            t_d.push((task, x))
                        }
                        Err(e) => {
                            error!("{}", e);
                            let _ = SVC_NATURE.task_dao.raw_to_error(&e, carrier);
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
        if SVC_NATURE.task_svc.create_batch_and_finish_carrier(&store_infos, &carrier.task_id).is_ok() {
            for task in t_d {
                let _ = CHANNEL_STORE.sender.lock().unwrap().send(task);
            }
        }
    }
}