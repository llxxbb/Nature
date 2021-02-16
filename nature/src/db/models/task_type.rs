use std::convert::TryFrom;

use crate::domain::*;

pub enum TaskType {
    Store = 1,
    Convert = 2,
    Batch = 11,
}

impl TryFrom<i8> for TaskType {
    type Error = NatureError;

    fn try_from(value: i8) -> Result<Self> {
        match value {
            1 => Ok(TaskType::Store),
            2 => Ok(TaskType::Convert),
            11 => Ok(TaskType::Batch),
            _ => Err(NatureError::VerifyError(format!("undefined [{}] for `TaskType`", value)))
        }
    }
}
