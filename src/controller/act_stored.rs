use nature_db::{RawTask, TaskDaoImpl};

use crate::actor::*;
use crate::task::{TaskForConvert, TaskForStore};

pub fn channel_stored(task: TaskForStore, raw: RawTask) {
    if task.next_mission.is_none() {
        let _ = TaskDaoImpl::delete(&&raw.task_id);
        return;
    }
    match TaskForConvert::gen_task(&task) {
        Ok(converters) => {
            let raws: Vec<RawTask> = converters.iter().map(|x| x.1.clone()).collect();
            if RawTask::save_batch(&raws, &raw.task_id, TaskDaoImpl::insert, TaskDaoImpl::delete).is_err() {
                return;
            }
            for t in converters {
                if t.0.target.delay == 0 {
                    let _ = ACT_CONVERT.try_send(MsgForTask(t.0, t.1));
                }
            }
        }
        Err(err) => {
            let _ = TaskDaoImpl::raw_to_error(&err, &raw);
            return;
        }
    }
}

