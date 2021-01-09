use std::env;
use std::ops::Deref;

use reqwest::Client;

use cfg::*;
use delay::*;
use sleep::*;

use crate::db::{D_T, RawTask, TaskDao};
use crate::domain::*;

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub async fn start() {
    dotenv::dotenv().ok();
    let _ = env_logger::init();
    let mut last_delay: u64 = 0;
    let base_delay = env::var("BASE_DELAY").unwrap_or_else(|_| "2".to_string()).parse::<i64>().unwrap();
    let load_size = env::var("LOAD_SIZE").unwrap_or_else(|_| "100".to_string()).parse::<i64>().unwrap();
    let clean_delay = env::var("CLEAN_DELAY").unwrap_or_else(|_| "2".to_string()).parse::<i64>().unwrap();
    info!("----------- {} : {}------------", "base_delay", base_delay);
    info!("----------- {} : {}------------", "load_size", load_size);
    info!("----------- {} : {}------------", "clean_delay", clean_delay);
    loop {
        last_delay = once(last_delay, base_delay, load_size, clean_delay).await
    }
}

async fn once(last_delay: u64, base_delay: i64, limit: i64, finish_delay: i64) -> u64 {
    debug!("start a new loop");
    let mut len = 0;
    let rs = D_T.get_overdue(base_delay, limit).await;
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
    match D_T.delete_finished(finish_delay).await {
        Ok(num) => info!("cleaned tasks : {}", num),
        Err(e) => warn!("clean task failed: {}", e)
    }
    sleep_by_records(len as u32, last_delay)
}


async fn process_delayed(r: &RawTask) -> () {
    debug!("process task: {:?}", r);
    let max_times = *MAX_RETRY_TIMES.deref();
    if (r.retried_times as usize) < max_times {
        let req = CLIENT.post(&*NATURE_SERVER_ADDRESS).json(r).send().await;
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

