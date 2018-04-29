use chrono::prelude::*;
use global::*;
use serde::Serialize;
use std::ops::Deref;
use util::*;
use uuid::UuidBytes;

/// carry every kinds of **Task Info** to process which stayed at `Ready` table
#[derive(Debug, Clone)]
pub struct Carrier<T> where T: Sized + Serialize {
    pub data: T,
    pub id: UuidBytes,
    pub create_time: i64,
    pub execute_time: i64,
}

impl<T> Carrier<T> where T: Sized + Serialize {
    pub fn new(task: T) -> Result<Carrier<T>> {
        // this can avoid regenerate same content with different id
        let new_id = generate_id(&task)?;
        Ok(Carrier {
            data: task,
            id: new_id,
            create_time: Local::now().timestamp_millis(),
            execute_time: Local::now().timestamp_millis(),
        })
    }

    /// Move Task Info from `Ready` to `Error` table
    pub fn move_to_error(_id: UuidBytes) -> Result<()> {
        // TODO
        unimplemented!()
    }
}

impl<T> Deref for Carrier<T> where T: Sized + Serialize {
    type Target = T;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.data
    }
}
