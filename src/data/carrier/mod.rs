use chrono::prelude::*;
use global::*;
use serde::Serialize;
use std::ops::Deref;
use util::*;
use uuid::UuidBytes;

/// carry every kinds of **Task Info** to process which stayed at `Ready` table
#[derive(Debug, Clone)]
pub struct Carrier<T> where T: Sized + Serialize {
    pub id: u128,
    pub create_time: i64,
    pub execute_time: i64,
    pub content: CarrierContent<T>,
}

#[derive(Debug, Clone)]
pub struct CarrierContent<T> {
    pub data: T,
    pub thing: String,
    pub data_type: u8,
}

impl<T> Carrier<T> where T: Sized + Serialize {
    pub fn new(task: T, thing: String, data_type: u8) -> Result<Carrier<T>> {
        // this can avoid regenerate same content with different id
        let new_id = generate_id(&task)?;
        Ok(Carrier {
            content: CarrierContent {
                data: task,
                thing,
                data_type,
            },
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
        &self.content.data
    }
}
