use nature_common::{NatureError, ParaForIDAndFrom, ParaForQueryByID, Result};
use nature_db::{InstanceDaoImpl, RawTask, StorePlanDaoImpl, TaskDaoImpl};

use crate::actor::{ACT_STORED, MsgForTask};
use crate::task::{CachedKey, TaskForStore};

pub fn channel_store(store: (TaskForStore, RawTask)) {
    let _ = save_instance(store.0, store.1);
}

pub fn save_instance(task: TaskForStore, carrier: RawTask) -> Result<()> {
    match InstanceDaoImpl::insert(&task.instance) {
        Ok(_) => do_instance_save(task, carrier),
        Err(NatureError::DaoDuplicated(_)) => duplicated_instance(task, carrier),
        Err(e) => Err(e)
    }
}

fn do_instance_save(task: TaskForStore, carrier: RawTask) -> Result<()> {
    let need_cache = task.need_cache;
    let key = &task.instance.get_unique();
    // if let Some(m) = &task.next_mission {
    //     for o in m {
    //         debug!("--saved instance: from:{},to:{}", task.instance.meta, o.to.meta_string());
    //     }
    // } else {
    //     debug!("----saved instance for meta : {} have no missions", task.instance.meta);
    // }
    ACT_STORED.try_send(MsgForTask(task, carrier))?;
    if need_cache {
        CachedKey::set(key);
    }
    Ok(())
}

fn duplicated_instance(task: TaskForStore, carrier: RawTask) -> Result<()> {
    // process meta which is not status----------------
    if task.instance.state_version == 0 {
        warn!("instance already exists, meta: {}, id: {}", task.instance.meta, task.instance.id);
        return do_instance_save(task, carrier);
    }
    // process status-meta-------------------

    let ins_from = task.instance.from.clone().unwrap();
    let para = ParaForIDAndFrom {
        id: task.instance.id,
        meta: task.instance.meta.clone(),
        from_id: ins_from.id,
        from_meta: ins_from.meta.clone(),
        from_state_version: ins_from.state_version,
        from_para: ins_from.para.clone(),
    };
    let old = InstanceDaoImpl::get_by_from(&para)?;
    if let Some(ins) = old {
        // same frominstance
        warn!("same source for meta: {}, replaced with old instance", &task.instance.meta);
        let task = TaskForStore::new(ins, task.next_mission.clone(), None, false);
        // maybe send failed for the previous process, so send it again, otherwise can't send it any more
        ACT_STORED.try_send(MsgForTask(task, carrier.clone()))?;
        return Ok(());
    } else {
        warn!("conflict for state-meta: [{}] on version : {}", &task.instance.meta, task.instance.state_version);
        let _ = StorePlanDaoImpl::delete(&ins_from.get_upstream(), &task.instance.meta)?;
        let ins = InstanceDaoImpl::get_by_id(&ParaForQueryByID::new(ins_from.id, &ins_from.meta))?;
        return match ins {
            Some(ins) => {
                let task = TaskForStore::new(ins, Some(vec![task.previous_mission.clone().unwrap()]), None, false);
                ACT_STORED.try_send(MsgForTask(task, carrier.clone()))?;
                Ok(())
            }
            None => {
                let error = NatureError::VerifyError("from-instance does not found".to_string());
                let _ = TaskDaoImpl::raw_to_error(&error, &carrier);
                Err(error)
            }
        };
    }
}
