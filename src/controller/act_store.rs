use nature_common::{NatureError, ParaForIDAndFrom, ParaForQueryByID, Result};
use nature_db::{InstanceDaoImpl, RawTask, StorePlanDaoImpl, TaskDaoImpl};

use crate::actor::{ACT_STORED, MsgForTask};
use crate::task::{CachedKey, TaskForStore};

pub fn channel_store(store: (TaskForStore, RawTask)) {
    let _ = save_instance(store.0, store.1);
}

pub fn save_instance(task: TaskForStore, carrier: RawTask) -> Result<()> {
    match InstanceDaoImpl::insert(&task.instance) {
        Ok(_) => {
            let need_cache = task.need_cache;
            let key = &task.instance.get_unique();
            if let Some(m) = &task.next_mission {
                for o in m {
                    debug!("--saved instance: from:{},to:{}", task.instance.meta, o.to.meta_string());
                }
            } else {
                debug!("----saved instance for meta : {} have no missions", task.instance.meta);
            }
            ACT_STORED.try_send(MsgForTask(task, carrier))?;
            if need_cache {
                CachedKey::set(key);
            }
            Ok(())
        }
        Err(NatureError::DaoDuplicated(err)) => duplicated_instance(&task, &carrier, err),
        Err(e) => Err(e)
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

