use std::ops::Deref;

use reqwest::Client;

use cfg::*;
use delay::*;
use sleep::*;
use crate::common::NatureError;

use crate::db::{D_T, RawTask, TaskDao};
use crate::util::logger::logger_init;

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub async fn start() {
    dotenv::dotenv().ok();
    logger_init();
    let mut last_delay: u64 = 0;
    info!("----------- {} : {}------------", "base_delay", *BASE_DELAY);
    info!("----------- {} : {}------------", "load_size", *LOAD_SIZE);
    info!("----------- {} : {}------------", "clean_delay", *CLEAN_DELAY);
    loop {
        last_delay = once(last_delay).await
    }
}

async fn once(last_delay: u64) -> u64 {
    debug!("start a new loop");
    let mut len = 0;
    let rs = D_T.get_overdue(*BASE_DELAY, *LOAD_SIZE).await;
    match rs {
        Ok(rs) => {
            len = rs.len();
            debug!("load tasks number: {}", rs.len());
            for r in rs {
                let _ = process_delayed(&r).await;
            }
        }
        Err(e) => {
            warn!("found error: {}", e)
        }
    }
    match D_T.delete_finished(*CLEAN_DELAY).await {
        Ok(num) => info!("cleaned tasks : {}", num),
        Err(e) => warn!("clean task failed: {}", e)
    }
    sleep_by_records(len as u32, last_delay)
}


async fn process_delayed(r: &RawTask) -> () {
    debug!("process task: {:?}", r);
    let max_times = *MAX_RETRY_TIMES.deref();
    if (r.retried_times as usize) < max_times {
        let req = CLIENT.post(&*REDO_URL).json(r).send().await;
        match req {
            Ok(_) => {
                debug!("send task succeed!");
                let delay = get_delay_by_times(r.retried_times);
                // 注释掉下一行可用于并发测试
                if let Err(e) = D_T.increase_times_and_delay(&r.task_id, delay).await {
                    warn!("task update failed: {}", e);
                }
            }
            Err(_) => {
                warn!("send task failed!");
            }
        }
    } else {
        debug!("tried too many times!");
        let _ = D_T.raw_to_error(&NatureError::EnvironmentError(format!("rtried over max times : {}", max_times)), r).await;
    }
}


pub mod cfg;
pub mod sleep;
mod delay;

