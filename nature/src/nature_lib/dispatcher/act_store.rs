use std::thread::sleep;
use std::time::Duration;

use actix_web::web::Data;

use crate::common::*;
use crate::db::{C_M, C_R, D_M, D_R, InstanceDaoImpl, MetaCache, Mission, RawTask, RelationCache};
use crate::db::flow_tool::{context_check, state_check};
use crate::domain::*;
use crate::nature_lib::dispatcher::channel_stored;
use crate::nature_lib::task::{CachedKey, TaskForConvert, TaskForStore};
use crate::nature_lib::task::gen_loop_mission;
use crate::util::web_context::WebContext;

pub async fn channel_store(task: TaskForStore, carrier: RawTask, context: Data<WebContext>) -> Result<()> {
    match InstanceDaoImpl::insert(&task.instance).await {
        Ok(_) => {
            // debug!("saved instance for: {}, task for: {:?}", &task.instance.meta, &task.next_mission);
            // the following after_saved can not be fired sometimes
            // tokio::spawn(async move {
            //     after_saved(task, carrier).await
            // });
            let _ = after_saved(task, carrier, context).await;
            Ok(())
        }
        Err(NatureError::DaoDuplicated(_)) => {
            duplicated_instance(task, carrier, context).await
        }
        Err(e) => Err(e)
    }
}

async fn after_saved(task: TaskForStore, carrier: RawTask, context: Data<WebContext>) -> Result<()> {
    let need_cache = task.need_cache;
    let key = &task.instance.get_key();
    channel_stored(task, carrier, context).await;
    if need_cache {
        CachedKey::set(key);
    }
    Ok(())
}

async fn duplicated_instance(task: TaskForStore, carrier: RawTask, context: Data<WebContext>) -> Result<()> {
    // process meta which is not status----------------
    if task.instance.path.state_version == 0 {
        warn!("instance already exists, meta: {}, id: {}", task.instance.path.meta, task.instance.id);
        return after_saved(task, carrier, context).await;
    }
    // process status-meta-------------------
    let ins_from = match task.instance.from.clone() {
        None => return Ok(()),
        Some(from) => from,
    };
    let para = IDAndFrom {
        id: task.instance.id,
        meta: task.instance.path.meta.clone(),
        from_key: ins_from.to_string(),
    };
    let old = InstanceDaoImpl::select_by_from(&para).await?;
    if let Some(ins) = old {
        // same from instance
        warn!("same source for meta: {}, replaced with old instance", &task.instance.path.meta);
        let task = TaskForStore::new(ins, task.next_mission.clone(), None, false);
        // maybe send failed for the previous process, so send it again, otherwise can't send it any more
        channel_stored(task, carrier.clone(), context).await;
        return Ok(());
    } else {
        warn!("conflict for state-meta: [{}] on version : {}", &task.instance.path.meta, task.instance.path.state_version);
        sleep(Duration::from_millis(10));
        let mut rtn = TaskForConvert::from_raw(&carrier, InstanceDaoImpl::select_by_id, &*C_M, &*D_M).await?;
        rtn.conflict_version = task.instance.path.state_version;
        context.sender.send((rtn, carrier, context.clone())).await?;
        Ok(())
    }
}

pub async fn get_store_task(instance: &Instance, previous_mission: Option<Mission>) -> Result<TaskForStore> {
    let meta = C_M.get(&instance.path.meta, &*D_M).await?;
    let meta_type = meta.get_meta_type();

    let mission = match meta_type {
        MetaType::Loop => {
            gen_loop_mission(instance, &*C_M, &*D_M).await?
        }
        _ => {
            let relations = C_R.get(&meta, &*D_R, &*C_M, &*D_M).await?;
            Mission::load_by_instance(instance, &relations, context_check, state_check)
        }
    };
    let task = TaskForStore::new(instance.clone(), mission, previous_mission, meta.need_cache());
    Ok(task)
}
