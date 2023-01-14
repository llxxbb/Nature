use actix_web::web::Data;
use crate::db::{D_T, RawTask, TaskDao};
use crate::nature_lib::task::{TaskForConvert, TaskForStore};
use crate::util::web_context::WebContext;

pub async fn channel_stored(task: TaskForStore, raw: RawTask, context :Data<WebContext>) {
    // for m in &task.next_mission {
    //     debug!("-- next mission: from:{}, to:{}", task.instance.meta, m.to.meta_string());
    // }
    if task.next_mission.is_empty() {
        let _ = D_T.finish_task(&&raw.task_id).await;
        return;
    }
    let next_task = TaskForConvert::gen_task(task);
    match next_task {
        Ok(converters) => {
            let mut raws: Vec<RawTask> = converters.iter().map(|x| x.1.clone()).collect();
            let rtn = RawTask::save_batch(&mut raws, &raw.task_id, &*D_T).await;
            if rtn.is_err() {
                warn!("==== converter task saved failed : {}", rtn.err().unwrap().to_string());
                return;
            }
            // notation! can't use retains `raws`, otherwise faild task would never be executed.
            let data = context.clone();
            tokio::spawn(async move {
                for t in converters {
                    if t.0.target.delay == 0 {
                        let _ = data.chanel.sender.lock().unwrap().send((t.0,t.1,context.clone()));
                    }
                }
            });
        }
        Err(err) => {
            warn!("{}", err);
            let _ = D_T.raw_to_error(&err, &raw).await;
            return;
        }
    }
}

