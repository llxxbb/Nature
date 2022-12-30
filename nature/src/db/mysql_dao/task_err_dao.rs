use mysql_async::params;

use crate::common::Result;
use crate::db::MySql;
use crate::db::raw_models::RawTaskError;
use crate::domain::task::TaskCondition;

lazy_static! {
    // Dao of Task
    pub static ref D_TE: TaskErrDaoImpl = TaskErrDaoImpl {};
}

#[async_trait]
pub trait TaskErrDao {
    async fn get(&self, para: &TaskCondition) -> Result<Vec<RawTaskError>>;
    async fn get_num(&self, task_for: &str) -> Result<u32>;
    async fn delete(&self, ids: &str) -> Result<u64>;
    async fn delete_for(&self, task_for: &str) -> Result<u64>;
    /// move to task table and redo it
    async fn reset(&self, id: &u64) -> Result<u64>;
}

pub struct TaskErrDaoImpl;

#[async_trait]
impl TaskErrDao for TaskErrDaoImpl {
    async fn get(&self, para: &TaskCondition) -> Result<Vec<RawTaskError>> {
        let sql = r"SELECT task_id, task_key, task_type, task_for, `data`, create_time, msg
            FROM task_error
            WHERE task_for = :task_for and task_id > :task_id
            ORDER by task_id
            limit :limit
            ";

        let p = params! {
            "task_for" => para.task_for.clone(),
            "task_id" => para.id_from,
            "limit" => para.limit,
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

    async fn reset(&self, id: &u64) -> Result<u64> {
        // dbg!(ids);
        let sql = format!("INSERT INTO task
            (task_id, task_key, task_type, task_for, task_state, `data`, retried_times, create_time, execute_time)
            SELECT
                task_id, task_key, task_type, task_for, 0 as task_state, `data`, 0 as retried_times, now(), now()
            FROM task_error te
             WHERE te.task_id={}", id);

        MySql::idu(&sql, ()).await
    }
}
