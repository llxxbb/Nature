use chrono::{Local, NaiveDateTime};
use mysql_async::params;

use crate::common::Result;
use crate::db::MySql;

pub struct TaskChecker;

impl TaskChecker {
    pub async fn check(cfg: &Condition) -> Result<usize> {
        let task_gt = if cfg.key_gt.eq("") { "" } else {
            " and task_key > :task_gt"
        };
        let task_lt = if cfg.key_lt.eq("") { "" } else {
            " and task_key < :task_lt"
        };
        // execute_time is closer to instance.create_time so does not use task.create_time.
        let time_ge = match cfg.time_ge {
            Some(_) => " and execute_time >= :time_ge",
            None => ""
        };
        let time_ge_v = match cfg.time_ge {
            Some(ge) => ge,
            None => Local::now().naive_local()
        };
        // create_time is closer to instance.create_time so does not use task.execute_time.
        let time_lt = match cfg.time_lt {
            Some(_) => " and create_time < :time_lt",
            None => ""
        };
        let time_lt_v = match cfg.time_lt {
            Some(lt) => lt,
            None => Local::now().naive_local()
        };
        let sql = format!("SELECT count(1) as num
                FROM task
                WHERE 1=1{}{}{}{}
                    and task_state = :state
            ", time_ge, time_lt, task_gt, task_lt);
        let p = params! {
            "task_gt" => cfg.key_gt.to_string(),
            "task_lt" => cfg.key_lt.to_string(),
            "time_ge" => time_ge_v,
            "time_lt" => time_lt_v,
            "state" => cfg.state,
        };
        let vec = MySql::fetch(sql, p, mysql_async::from_row).await?;
        Ok(vec[0])
    }
}

pub struct Condition {
    pub key_gt: String,
    pub key_lt: String,
    pub time_ge: Option<NaiveDateTime>,
    pub time_lt: Option<NaiveDateTime>,
    pub state: i8,
}

#[cfg(test)]
mod test {
    use std::env;

    use chrono::{Local, TimeZone};

    use crate::db::CONN_STR;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn get_test() {
        env::set_var("DATABASE_URL", CONN_STR);
        let _ = env_logger::init();

        let condition = Condition {
            key_gt: "".to_string(),
            key_lt: "".to_string(),
            time_ge: Some(Local::now().naive_local()),
            time_lt: Some(Local::now().naive_local()),
            state: 1,
        };
        let num = TaskChecker::check(&condition).await.unwrap();
        assert_eq!(0, num)
    }

    #[tokio::test]
    #[ignore]
    async fn get_ignore_test() {
        env::set_var("DATABASE_URL", CONN_STR);
        let _ = env_logger::init();

        let condition = Condition {
            key_gt: "B:sale/item/count:1|0|".to_string(),
            key_lt: "B:sale/item/count:2|0|".to_string(),
            time_ge: Some(Local.ymd(2020, 8, 7).and_hms(0, 0, 0).naive_local()),
            time_lt: Some(Local::now().naive_local()),
            state: 1,
        };
        let num = TaskChecker::check(&condition).await.unwrap();
        assert_eq!(6, num)
    }
}