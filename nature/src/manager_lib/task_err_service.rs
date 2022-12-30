use crate::db::{D_T, D_TE, TaskDao, TaskErrDao};
use crate::common::*;

pub struct TaskErrService;

impl TaskErrService {
    pub async fn move_to_task(ids: Vec<u64>) -> Result<usize> {
        let len = ids.len();
        for id in ids {
            match D_TE.reset(&id).await {
                Err(NatureError::DaoDuplicated(_)) => {
                    D_T.reset(&id).await?;
                }
                Err(e) => return Err(e),
                _ => {}
            }
        }
        Ok(len)
    }
}