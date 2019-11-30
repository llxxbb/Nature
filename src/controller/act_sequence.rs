use nature_common::{NatureError, Result, TaskForSerial};
use nature_db::{InstanceDaoImpl, MetaCacheImpl, MetaDaoImpl, Mission, RawTask, RelationCacheImpl, RelationDaoImpl, TaskDaoImpl, TaskType};

use crate::actor::{MsgForTask,ACT_STORED};
use crate::task::{TaskForSerialWrapper, TaskForStore};

pub fn channel_serial(task: MsgForTask<TaskForSerial>) {
    if let Err(e) = inner_serial(&task) {
        let _ = TaskDaoImpl::raw_to_error(&e, &task.1);
    }
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