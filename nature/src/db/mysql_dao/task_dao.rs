use chrono::{Duration, Local};
use mysql_async::{params, Params};

use crate::common::*;
use crate::db::MySql;
use crate::db::raw_models::{RawTask, RawTaskError};

lazy_static! {
    pub static ref D_T: TaskDaoImpl = TaskDaoImpl {};
}

#[async_trait]
pub trait TaskDao {
    async fn insert(&self, raw: &RawTask) -> Result<u64>;
    async fn delete(&self, record_id: &u64) -> Result<u64>;
    async fn delete_finished(&self, delay: i64) -> Result<u64>;
    async fn raw_to_error(&self, err: &NatureError, raw: &RawTask) -> Result<u64>;
    async fn get_overdue(&self, delay: i64, limit: i64) -> Result<Vec<RawTask>>;
    async fn update_execute_time(&self, record_id: &u64, delay: i64) -> Result<u64>;
    async fn finish_task(&self, record_id: &u64) -> Result<u64>;
    async fn increase_times_and_delay(&self, _record_id: &u64, delay: i32) -> Result<u64>;
    async fn get(&self, record_id: &u64) -> Result<Option<RawTask>>;
    // reset task to unfinished and runtimes to 0
    async fn reset(&self, task_id: &u64) -> Result<u64>;
}

pub struct TaskDaoImpl;

#[async_trait]
impl TaskDao for TaskDaoImpl {
    async fn insert(&self, raw: &RawTask) -> Result<u64> {
        let sql = r"INSERT INTO task
            (task_id, task_key, task_type, task_for, task_state, `data`, create_time, execute_time, retried_times)
            VALUES(:task_id, :task_key, :task_type, :task_for, :task_state, :data, :create_time, :execute_time, :retried_times)";

        let p: Params = raw.clone().into();
        let num: u64 = match MySql::idu(sql, p).await {
            Ok(n) => {
                debug!("---- saved task KEY: {} FOR: {} TYPE: {}", &raw.task_key, &raw.task_for, raw.task_type);
                n
            }
            Err(e) => match e {
                NatureError::DaoDuplicated(_) => {
                    warn!("==== task repeated. KEY: {} FOR: {} TYPE: {}", &raw.task_key, &raw.task_for, raw.task_type);
                    0
                }
                _ => return {
                    warn!("**** task insert error. KEY: {} FOR: {} TYPE: {} err: {}", &raw.task_key, &raw.task_for, raw.task_type, e);
                    Err(e)
                }
            }
        };
        Ok(num)
    }

    #[allow(dead_code)]
    async fn delete(&self, record_id: &u64) -> Result<u64> {
        let sql = r"DELETE FROM task
            WHERE task_id=:task_id";

        let p = params! {
            "task_id" => record_id,
        };

        let rtn = MySql::idu(sql, p).await?;
        Ok(rtn)
    }

    /// delete finished task after `delay` seconds
    async fn delete_finished(&self, delay: i64) -> Result<u64> {
        let sql = r"DELETE FROM task
            WHERE execute_time < date_sub(now(), interval :delay second) AND task_state = 1";

        let p = params! {
            "delay" => delay,
        };

        let rtn = MySql::idu(sql, p).await?;
        Ok(rtn)
    }

    async fn raw_to_error(&self, err: &NatureError, raw: &RawTask) -> Result<u64> {
        let sql = r"INSERT INTO task_error
            (task_id, task_key, task_type, task_for, `data`, create_time, msg)
            VALUES(:task_id, :task_key, :task_type, :task_for, :data, :create_time, :msg)";

        let rd = RawTaskError::from_raw(err, raw);
        let p: Params = rd.into();
        let num = match MySql::idu(sql, p).await {
            Ok(num) => {
                self.delete(&raw.task_id).await?;
                num
            }
            Err(NatureError::DaoDuplicated(_)) => {
                self.delete(&raw.task_id).await?;
                0
            }
            Err(e) => return Err(e)
        };
        Ok(num)
    }

    async fn get_overdue(&self, delay: i64, limit: i64) -> Result<Vec<RawTask>> {
        let sql = r"SELECT task_id, task_key, task_type, task_for, task_state, `data`, create_time, execute_time, retried_times
            FROM task
            WHERE execute_time < :execute_time and task_state = 0
            LIMIT :limit";

        let _execute_time = Local::now().checked_add_signed(Duration::seconds(delay)).unwrap().naive_local();
        let p = params! {
            "execute_time" => _execute_time,
            "limit" => limit,
        };

        MySql::fetch(sql, p, RawTask::from).await
    }

    async fn update_execute_time(&self, record_id: &u64, delay: i64) -> Result<u64> {
        let sql = r"UPDATE task
            SET execute_time=:execute_time
            WHERE task_id=:task_id";

        let _time = Local::now().checked_add_signed(Duration::seconds(delay)).unwrap().naive_local();
        let p = params! {
            "execute_time" => _time,
            "task_id" => record_id,
        };
        let rtn = MySql::idu(sql, p).await?;
        Ok(rtn)
    }

    async fn finish_task(&self, record_id: &u64) -> Result<u64> {
        let sql = r"UPDATE task
            SET task_state=1
            WHERE task_id=:task_id and task_state=0";

        let p = params! {
            "task_id" => record_id,
        };
        let rtn = match MySql::idu(sql, p).await {
            Ok(n) => n,
            Err(e) => {
                warn!("**** save task error : {}", record_id);
                return Err(e);
            }
        };
        Ok(rtn)
    }

    /// increase one times and delay `delay` seconds
    async fn increase_times_and_delay(&self, _record_id: &u64, delay: i32) -> Result<u64> {
        let sql = r"UPDATE task
            SET execute_time=:execute_time, retried_times = retried_times+1
            WHERE task_id=:task_id";

        let _time = Local::now().checked_add_signed(Duration::seconds(delay as i64)).unwrap().naive_local();
        let p = params! {
            "execute_time" => _time,
            "task_id" => _record_id,
        };
        let rtn = MySql::idu(sql, p).await?;
        Ok(rtn)
    }

    async fn get(&self, record_id: &u64) -> Result<Option<RawTask>> {
        let sql = r"SELECT task_id, task_key, task_type, task_for, task_state, `data`, create_time, execute_time, retried_times
            FROM task
            WHERE task_id=:task_id";

        let p = params! {
            "task_id" => record_id,
        };

        let rtn = MySql::fetch(sql, p, RawTask::from).await?;
        match rtn.len() {
            0 => Ok(None),
            1 => Ok(Some(rtn[0].clone())),
            _ => Err(NatureError::SystemError("should less than 2 record return".to_string())),
        }
    }

    async fn reset(&self, task_id: &u64) -> Result<u64> {
        let sql = r"update task set task_state=0, retried_times=0
            WHERE task_id=:task_id";

        let p = params! {
            "task_id" => task_id,
        };

        let rtn = MySql::idu(sql, p).await?;
        Ok(rtn)
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::db::CONN_STR;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn insert_repeat_test() {
        env::set_var("DATABASE_URL", CONN_STR);
        let mut task = RawTask::default();
        let _num = D_T.delete(&1).await.unwrap();
        let num = D_T.insert(&task).await.unwrap();
        task.task_id = num;
        assert_eq!(1, num);
        // repeat
        let num = D_T.insert(&task).await.unwrap();
        assert_eq!(0, num);
        let get_task = D_T.get(&1).await.unwrap();
        assert!(get_task.is_some());
        let num = D_T.raw_to_error(&NatureError::LogicalError("my test".to_string()), &task).await.unwrap();
        assert_eq!(1, num);
        let get_task = D_T.get(&1).await.unwrap();
        assert!(get_task.is_none());
    }
}