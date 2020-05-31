use std::thread::sleep;
use std::time::Duration;

use nature_common::{IDAndFrom, NatureError, Result};
use nature_db::{InstanceDaoImpl, MCG, MG, RawTask};

use crate::channels::CHANNEL_CONVERT;
use crate::controller::channel_stored;
use crate::task::{CachedKey, TaskForConvert, TaskForStore};

pub async fn channel_store(task: TaskForStore, carrier: RawTask) -> Result<()> {
    match InstanceDaoImpl::insert(&task.instance).await {
        Ok(_) => do_instance_save(task, carrier).await,
        Err(NatureError::DaoDuplicated(_)) => duplicated_instance(task, carrier).await,
        Err(e) => Err(e)
    }
}

async fn do_instance_save(task: TaskForStore, carrier: RawTask) -> Result<()> {
    let need_cache = task.need_cache;
    let key = &task.instance.get_key();
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
    let para = IDAndFrom {
        id: task.instance.id,
        meta: task.instance.meta.clone(),
        from_key: ins_from.to_string(),
    };
    let old = InstanceDaoImpl::get_by_from(&para).await?;
    if let Some(ins) = old {
        // same frominstance
        warn!("same source for meta: {}, replaced with old instance", &task.instance.meta);
        let task = TaskForStore::new(ins, task.next_mission.clone(), None, false);
        // maybe send failed for the previous process, so send it again, otherwise can't send it any more
        channel_stored(task, carrier.clone()).await;
        return Ok(());
    } else {
        warn!("conflict for state-meta: [{}] on version : {}", &task.instance.meta, task.instance.state_version);
        sleep(Duration::from_millis(10));
        let mut rtn = TaskForConvert::from_raw(&carrier, InstanceDaoImpl::get_by_key, MCG, MG).await?;
        rtn.conflict_version = task.instance.state_version;
        CHANNEL_CONVERT.sender.lock().unwrap().send((rtn, carrier))?;
        Ok(())
    }
}
