use std::rc::Rc;

use nature_db::task_type::TaskType;

use crate::flow::store::StoreServiceTrait;

use super::*;

pub trait ParallelServiceTrait {
    fn parallel(&self, batch: ParallelBatchInstance) -> Result<()>;
}

pub struct ParallelServiceImpl {
    pub task_svc: Rc<TaskServiceTrait>,
    pub store: Rc<StoreServiceTrait>,
}

impl ParallelServiceTrait for ParallelServiceImpl {
    fn parallel(&self, batch: ParallelBatchInstance) -> Result<()> {
        let raw = RawTask::new(&batch, &batch.thing.get_full_key(), TaskType::ParallelBatch as i16)?;
        match TaskDaoImpl::insert(&raw) {
            Ok(_carrier) => {
                // to process asynchronous
                let _ = CHANNEL_PARALLEL.sender.lock().unwrap().send((batch, raw));
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

impl ParallelServiceImpl {
    pub fn save<FI, FD>(batch: ParallelBatchInstance, carrier: RawTask, raw_insert: FI, raw_delete: FD)
        where FI: Fn(&RawTask) -> Result<usize>, FD: Fn(&[u8]) -> Result<usize>,
    {
        let mut tasks: Vec<RawTask> = Vec::new();
        let mut tuple: Vec<(StoreTaskInfo, RawTask)> = Vec::new();
        for instance in batch.instances.iter() {
            match StoreTaskInfo::gen_task(&instance, OneStepFlowCacheImpl::get, Mission::filter_relations) {
                Ok(task) => {
                    match RawTask::new(&task, &instance.thing.get_full_key(), TaskType::Store as i16) {
                        Ok(car) => {
                            tasks.push(car.clone());
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
        if RawTask::save_batch(&tasks, &carrier.task_id, raw_insert, raw_delete).is_err() {
            return;
        }
        for c in tuple {
            let _ = CHANNEL_STORE.sender.lock().unwrap().send(c);
        }
    }
}