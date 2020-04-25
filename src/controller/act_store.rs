use nature_common::{NatureError, ParaForIDAndFrom, Result};
use nature_db::{INS_KEY_GETTER, InstanceDaoImpl, MCG, MG, RawTask};

use crate::channels::CHANNEL_CONVERT;
use crate::controller::channel_stored;
use crate::task::{CachedKey, TaskForConvert, TaskForStore};

pub async fn channel_store(task: TaskForStore, carrier: RawTask) -> Result<()> {
    match InstanceDaoImpl::insert(&task.instance) {
        Ok(_) => do_instance_save(task, carrier).await,
        Err(NatureError::DaoDuplicated(_)) => duplicated_instance(task, carrier).await,
        Err(e) => Err(e)
    }
}

async fn do_instance_save(task: TaskForStore, carrier: RawTask) -> Result<()> {
    let need_cache = task.need_cache;
    let key = &task.instance.get_unique();
    // for o in &task.next_mission {
    //     debug!("--saved instance: from:{},to:{}", task.instance.meta, o.to.meta_string());
    // }
    channel_stored(task, carrier).await;
    if need_cache {
        CachedKey::set(key);
    }
    Ok(())
}

async fn duplicated_instance(task: TaskForStore, carrier: RawTask) -> Result<()> {
    // process meta which is not status----------------
    if task.instance.state_version == 0 {
        warn!("instance already exists, meta: {}, id: {}", task.instance.meta, task.instance.id);
        return do_instance_save(task, carrier).await;
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
        channel_stored(task, carrier.clone()).await;
        return Ok(());
    } else {
        warn!("conflict for state-meta: [{}] on version : {}", &task.instance.meta, task.instance.state_version);
        let rtn = TaskForConvert::from_raw(&carrier, INS_KEY_GETTER, MCG, MG)?;
        CHANNEL_CONVERT.sender.lock().unwrap().send((rtn, carrier))?;
        // do_convert(rtn, carrier).await;
        Ok(())
    }
}
