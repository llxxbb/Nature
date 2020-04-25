use nature_db::{RawTask, TaskDaoImpl};

use crate::channels::CHANNEL_CONVERT;
use crate::task::{TaskForConvert, TaskForStore};

pub async fn channel_stored(task: TaskForStore, raw: RawTask) {
    // if let Some(vm) = &task.next_mission {
    //     for m in vm {
    //         debug!("stored task: from:{}, to:{}", task.instance.meta, m.to.meta_string());
    //     }
    // }
    if task.next_mission.is_empty() {
        let _ = TaskDaoImpl::finish_task(&&raw.task_id);
        return;
    }
    match TaskForConvert::gen_task(&task) {
        Ok(converters) => {
            let raws: Vec<RawTask> = converters.iter().map(|x| x.1.clone()).collect();
            if RawTask::save_batch(&raws, &raw.task_id, TaskDaoImpl::insert, TaskDaoImpl::finish_task).is_err() {
                return;
            }
            for t in converters {
                if t.0.target.delay == 0 {
                    // debug!("--generated convert task: from:{}, to:{}", t.0.from.meta, t.0.target.to.meta_string());
                    let _ = CHANNEL_CONVERT.sender.lock().unwrap().send(t);
                    // do_convert(t.0, t.1).await;
                }
            }
        }
        Err(err) => {
            let _ = TaskDaoImpl::raw_to_error(&err, &raw);
            return;
        }
    }
}

