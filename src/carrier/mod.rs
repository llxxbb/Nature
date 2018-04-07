use define::*;
use std::ops::Deref;
//use uuid::UuidBytes;

/// carry every kinds of **Task Info** to process which stayed at `Ready` table
#[derive(Debug)]
pub struct Carrier<T> {
    pub data: T,
//    _id: UuidBytes,
//    _transmitted_times: u8,
//    _create_time: u64,
//    _execute_time: u64,
}

impl<T> Carrier<T> {
    /// Move Task Info from `Ready` to `Error` table
    pub fn drop(_task: &T) -> Result<()> {
        // TODO
        unimplemented!()
    }
}

impl<T> Deref for Carrier<T> {
    type Target = T;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.data
    }
}
