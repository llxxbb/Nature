use nature_common::{Instance, MetaType, Result};
use nature_db::{C_M, C_R, D_M, D_R, D_T, MetaCache, Mission, RawTask, RelationCache, TaskDao};
use nature_db::flow_tool::{context_check, state_check};

use crate::controller::channel_store;
use crate::task::{gen_loop_mission, TaskForStore};

pub async fn channel_batch(instances: Vec<Instance>, raw: RawTask) {
    if let Err(e) = inner_batch(instances, &raw).await {
        warn!("insert batch error: {}", e);
        let _ = D_T.raw_to_error(&e, &raw).await;
    }
}

async fn inner_batch(instances: Vec<Instance>, raw: &RawTask) -> Result<()> {
    let mut store_infos: Vec<RawTask> = Vec::new();
    let mut t_d: Vec<(TaskForStore, RawTask)> = Vec::new();
    for instance in &instances {
        let meta = C_M.get(&instance.meta, &*D_M).await?;
        let mission = match meta.get_meta_type() {
            MetaType::Loop => {
                gen_loop_mission(instance, &*C_M, &*D_M).await?
            }
            _ => {
                let r = C_R.get(&instance.meta, &*D_R, &*C_M, &*D_M).await?;
                Mission::get_by_instance(&instance, &r, context_check, state_check)
            }
        };
        let task = TaskForStore::new(instance.clone(), mission, None, meta.need_cache());
        match task.to_raw() {
            Ok(x) => {
                store_infos.push(x.clone());
                t_d.push((task, x))
            }
            Err(e) => return Err(e)
        }
    }
    if RawTask::save_batch(&mut store_infos, &raw.task_id, &*D_T).await.is_ok() {
        for task in t_d {
            // if let Some(m) = &task.0.next_mission {
            //     for o in m {
            //         debug!("--store task generated: from:{},to:{}", task.0.instance.meta, o.to.meta_string());
            //     }
            // } else {
            //     debug!("----meta : {} have no missions", task.0.instance.meta);
            // }
            let _ = channel_store(task.0, task.1).await;
        }
    }
    Ok(())
}
