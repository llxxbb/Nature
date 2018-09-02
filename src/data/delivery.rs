use nature_common::NatureError;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::Deref;

/// carry every kinds of **Task Info** to process which stayed at `Ready` table
#[derive(Debug, Clone)]
pub struct Carrier<T> where T: Sized + Serialize + Debug {
    pub id: u128,
    pub create_time: i64,
    pub execute_time: i64,
    pub content: CarrierContent<T>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CarrierContent<T> {
    pub data: T,
    pub thing: String,
    pub data_type: u8,
}


impl<T> Deref for Carrier<T> where T: Sized + Serialize + Debug {
    type Target = T;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.content.data
    }
}


#[derive(Debug)]
pub struct CarryError<'a, T: 'a> where T: Sized + Serialize + Debug {
    pub err: NatureError,
    pub carrier: &'a Carrier<T>,
}

pub enum DataType {
    Store = 1,
    Dispatch = 2,
    Convert = 3,
    ParallelBatch = 11,
    QueueBatch = 12,
}
