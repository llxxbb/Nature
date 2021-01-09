use crate::db::{D_T, RawTask, TaskDao};
use crate::nature_lib::dispatcher::{channel_store, get_store_task};
use crate::domain::*;
use crate::nature_lib::task::TaskForStore;

pub async fn channel_batch(instances: Vec<Instance>, raw: RawTask) {
    if let Err(e) = inner_batch(instances, &raw).await {
        warn!("insert batch error: {}", e);
        let _ = D_T.raw_to_error(&e, &raw).await;
    }
}

async fn inner_batch(instances: Vec<Instance>, raw: &RawTask) -> Result<()> {
    let mut store_info_vec: Vec<RawTask> = Vec::new();
    let mut t_d: Vec<(TaskForStore, RawTask)> = Vec::new();
    for instance in &instances {
        let task = get_store_task(&instance, None).await?;
        match task.to_raw() {
            Ok(x) => {
                store_info_vec.push(x.clone());
                t_d.push((task, x))
            }
            Err(e) => return Err(e)
        }
    }
    if RawTask::save_batch(&mut store_info_vec, &raw.task_id, &*D_T).await.is_ok() {
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
