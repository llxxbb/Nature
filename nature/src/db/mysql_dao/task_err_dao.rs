use mysql_async::params;

use crate::db::MySql;
use crate::db::raw_models::RawTaskError;
use crate::domain::*;

lazy_static! {
    // Dao of Task
    pub static ref D_TE: TaskErrDaoImpl = TaskErrDaoImpl {};
}

#[async_trait]
pub trait TaskErrDao {
    async fn get(&self, task_for: &str, limit: i32, from: u64) -> Result<Vec<RawTaskError>>;
    async fn get_num(&self, task_for: &str) -> Result<u32>;
    async fn delete(&self, ids: &str) -> Result<u64>;
    async fn delete_for(&self, task_for: &str) -> Result<u64>;
    /// move to task table and redo it
    async fn reset(&self, ids: &str) -> Result<u64>;
}

pub struct TaskErrDaoImpl;

#[async_trait]
impl TaskErrDao for TaskErrDaoImpl {
    async fn get(&self, task_for: &str, limit: i32, from: u64) -> Result<Vec<RawTaskError>> {
        let sql = r"SELECT task_id, task_key, task_type, task_for, `data`, create_time, msg
            FROM task_error
            WHERE task_for = :task_for and task_id > :task_id
            ORDER by task_id
            limit :limit
            ";

        let p = params! {
            "task_for" => task_for,
            "task_id" => from,
            "limit" => limit,
        };

        let rtn = MySql::fetch(sql, p, RawTaskError::from).await?;
        Ok(rtn)
    }

    async fn get_num(&self, task_for: &str) -> Result<u32> {
        let sql = r"SELECT count(1) as cnt
            FROM task_error
            WHERE task_for = :task_for";

        let p = params! {
            "task_for" => task_for,
        };

        MySql::count(sql, p).await
    }

    async fn delete(&self, ids: &str) -> Result<u64> {
        let sql = r"DELETE FROM task_error
            WHERE task_id in (:ids)";

        let p = params! {
            "ids" => ids,
        };

        MySql::idu(sql, p).await
    }

    async fn delete_for(&self, task_for: &str) -> Result<u64> {
        let sql = r"DELETE FROM task_error
            WHERE task_for = :task_for";

        let p = params! {
            "task_for" => task_for,
        };

        MySql::idu(sql, p).await
    }

    async fn reset(&self, ids: &str) -> Result<u64> {
        let sql = r"INSERT INTO task
            (task_id, task_key, task_type, task_for, task_state, `data`,  retried_times)
            SELECT
                task_id, task_key, task_type, task_for, 0 as task_state, `data`, 0 as retried_times
            FROM task_error
             WHERE task_id in (:ids)";

        let p = params! {
            "ids" => ids,
        };

        MySql::idu(sql, p).await
    }
}
