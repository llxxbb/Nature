use nature_db::{D_T, RawTask, TaskDao};

use crate::channels::CHANNEL_CONVERT;
use crate::task::{TaskForConvert, TaskForStore};

pub async fn channel_stored(task: TaskForStore, raw: RawTask) {
    // for m in &task.next_mission {
    //     debug!("-- next mission: from:{}, to:{}", task.instance.meta, m.to.meta_string());
    // }
    if task.next_mission.is_empty() {
        let _ = D_T.finish_task(&&raw.task_id).await;
        return;
    }
    match TaskForConvert::gen_task(&task) {
        Ok(converters) => {
            let mut raws: Vec<RawTask> = converters.iter().map(|x| x.1.clone()).collect();
            let rtn = RawTask::save_batch(&mut raws, &raw.task_id, &*D_T).await;
            if rtn.is_err() {
                warn!("==== converter task saved failed : {}", rtn.err().unwrap().to_string());
                return;
            }
            for t in converters {
                if t.0.target.delay == 0 {
                    let _ = CHANNEL_CONVERT.sender.lock().unwrap().send(t);
                }
            }
        }
        Err(err) => {
            warn!("{}", err);
            let _ = D_T.raw_to_error(&err, &raw).await;
            return;
        }
    }
}

