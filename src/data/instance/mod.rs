extern crate r2d2;

use chrono::prelude::*;
use data::thing::*;
use define::*;
#[cfg(not(test))]
pub use self::instance_impl::*;
pub use self::instance_trait::*;
#[cfg(test)]
pub use self::mock::*;
use service::*;
use uuid::*;

/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Instance {
    /// A unique value used to distinguish other instance
    pub id: UuidBytes,
    pub data: InstanceNoID,
}




/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InstanceNoID {
    /// This instance's Type
    pub thing: Thing,
    /// The time which plan to flow for this instance
    pub execute_time: i64,
    /// When this instance created
    pub create_time: i64,
    /// What contend in this instance for the `Thing`
    pub content: String,
    /// Is a json for a `Map[key, value]` which contents other instance for other `Thing`'s.
    /// `Nature` can transform those to `Instance`'s by flowing.
    ///
    /// # Key
    ///
    /// context name
    ///
    /// # Value
    ///
    /// json data for a `Instance`.
    pub context: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ParallelBatchInstance(Vec<Instance>);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SerialBatchInstance {
    pub context_for_finish: String,
    pub ignore_error: bool,
    pub instance: Vec<Instance>,
}

pub mod instance_trait;
pub mod instance_impl;
#[cfg(test)]
pub mod mock;
