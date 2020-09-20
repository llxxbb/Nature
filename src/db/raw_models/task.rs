use std::collections::HashSet;
use std::fmt::Debug;

use chrono::prelude::*;
use lazy_static::__Deref;
use mysql_async::{params, Row, Value};
use serde::Serialize;

use crate::common::*;
use crate::db::models::define::*;
use crate::db::TaskDao;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct RawTask {
    pub task_id: u64,
    pub task_key: String,
    pub task_type: i8,
    pub task_for: String,
    pub task_state: i8,
    pub data: String,
    pub create_time: NaiveDateTime,
    pub execute_time: NaiveDateTime,
    pub retried_times: i16,
}

impl Default for RawTask {
    fn default() -> Self {
        RawTask {
            task_id: 0,
            task_key: "".to_string(),
            task_type: 0,
            task_for: "".to_string(),
            task_state: 0,
            data: "".to_string(),
            create_time: Local::now().naive_local(),
            execute_time: Local::now().naive_local(),
            retried_times: 0,
        }
    }
}

impl RawTask {
    pub fn new<T: Serialize + Debug>(task: &T, task_key: &str, task_type: i8, task_for: &str) -> Result<RawTask> {
        let json = serde_json::to_string(task)?;
        Self::from_str(&json, task_key, task_type, task_for)
    }

    pub fn from_str(json: &str, task_key: &str, task_type: i8, task_for: &str) -> Result<RawTask> {
        if json.len() > *TASK_CONTENT_MAX_LENGTH.deref() {
            return Err(NatureError::SystemError("data's length can' be over : ".to_owned() + &TASK_CONTENT_MAX_LENGTH.to_string()));
        }
        let time = Local::now().naive_local();
        Ok(RawTask {
            task_id: 0,
            task_key: task_key.to_string(),
            task_type,
            task_for: task_for.to_string(),
            task_state: 0,
            data: json.to_string(),
            create_time: time,
            execute_time: time,
            retried_times: 0,
        })
    }


    /// for performance reason, one-to-one carry which we can reuse the beginning carry to finish all flows.
    /// That way we need not to communicate with DB for create new and delete old carrier.
    /// But for failure we must redo from beginning. but I think it has small chance.
    /// Another disadvantage is the failure information will be attached to the beginning.
    pub fn finish_old<FI, FD>(&mut self, old: &RawTask, _dao_insert: FI, _dao_delete: FD) -> Result<usize>
        where FI: Fn(&RawTask) -> Result<usize>,
              FD: Fn(&[u8]) -> Result<usize>
    {
        // TODO  当遇到错误时如果要结束的 delivery ID 和新的delivery 不一样 需要结束之前的 delivery 并创建新的 delivery
        self.task_id = old.task_id.clone(); // the id is used for final finished
        Ok(1)
    }


    pub async fn save_batch<T>(news: &mut Vec<RawTask>, old_id: &u64, task: &T) -> Result<()>
        where T: TaskDao
    {
        let mut will_deleted: HashSet<RawTask> = HashSet::new();
        for v in news.iter_mut() {
            let num = task.insert(&v).await?;
            // drop repeated task avoid data consistent problem, retry.exe will pick it up
            if num < 1 {
                will_deleted.insert(v.clone());
            }
        }
        news.retain(|one| will_deleted.get(&one) != Some(&one));
        task.finish_task(old_id).await?;
        Ok(())
    }

    pub fn task_string(&self) -> String {
        format!("raw_task: key|type|for {}{}{}", self.task_key, self.task_type, self.task_for)
    }
}

impl From<Row> for RawTask {
    fn from(row: Row) -> Self {
        let (task_id, task_key, task_type, task_for, task_state, data, create_time, execute_time, retried_times) = mysql_async::from_row(row);
        RawTask {
            task_id,
            task_key,
            task_type,
            task_for,
            task_state,
            data,
            create_time,
            execute_time,
            retried_times,
        }
    }
}

impl Into<Vec<(String, Value)>> for RawTask {
    fn into(self) -> Vec<(String, Value)> {
        params! {
            "task_id" => self.task_id,
            "task_key" => self.task_key,
            "task_type" => self.task_type,
            "task_for" => self.task_for,
            "task_state" => self.task_state,
            "data" => self.data,
            "create_time" => self.create_time,
            "execute_time" => self.execute_time,
            "retried_times" => self.retried_times,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[derive(Clone, Eq, Hash, PartialEq)]
    struct MyTest(String);

    #[test]
    #[ignore]
    fn vec_test() {
        let mut input: Vec<MyTest> = vec![
            MyTest("a".to_string()),
            MyTest("b".to_string()),
            MyTest("a".to_string()),
        ];
        let mut will_deleted: HashSet<MyTest> = HashSet::new();
        for v in input.iter() {
            if v.0 == "b" {
                will_deleted.insert(v.clone());
            }
        }
        input.retain(|one| will_deleted.get(&one) != Some(&one));
        assert_eq!(2, input.len());
    }
}