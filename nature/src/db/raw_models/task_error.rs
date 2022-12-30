use chrono::prelude::*;
use mysql_async::{params, Params, Row};

use crate::common::NatureError;
use crate::db::raw_models::RawTask;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct RawTaskError {
    pub task_id: u64,
    pub task_key: String,
    pub task_type: i8,
    pub task_for: String,
    pub data: String,
    pub create_time: NaiveDateTime,
    pub msg: String,
}

impl RawTaskError {
    pub fn from_raw(err: &NatureError, raw: &RawTask) -> Self {
        RawTaskError {
            task_id: raw.task_id,
            task_key: raw.task_key.clone(),
            task_type: raw.task_type,
            data: raw.data.clone(),
            create_time: raw.create_time,
            msg: format!("{:?}", err),
            task_for: "".to_string(),
        }
    }
}


impl From<Row> for RawTaskError {
    fn from(row: Row) -> Self {
        let (task_id, task_key, task_type, task_for, data, create_time, msg) = mysql_async::from_row(row);
        RawTaskError {
            task_id,
            task_key,
            task_type,
            task_for,
            data,
            create_time,
            msg,
        }
    }
}

impl Into<Params> for RawTaskError {
    fn into(self) -> Params {
        params! {
            "task_id" => self.task_id,
            "task_key" => self.task_key,
            "task_type" => self.task_type,
            "task_for" => self.task_for,
            "data" => self.data,
            "create_time" => self.create_time,
            "msg" => self.msg,
        }
    }
}
