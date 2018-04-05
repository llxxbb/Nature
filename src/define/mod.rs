extern crate diesel;

use chrono::prelude::*;
pub use self::error::*;
use std;


///! A public lib for outer user call


/// `Thing`'s basic information
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Default, Clone, Ord, PartialOrd)]
pub struct Thing {
    /// # Identify a `Thing`.
    ///
    /// A `Thing` may have a lots of `ThingInstance`s, so it's a **Class** for ThingInstance`.
    /// Because there are huge quantity of `Thing`s , so we need a way to organize `Thing`s.
    /// A way is to set name with hierarchical structures,
    ///
    /// # Value Example
    ///
    /// shop/order
    pub key: String,

    /// A `Thing` can be changed in future, the `version` will support this without effect the old ones
    pub version: u32,
}

/// `Thing`'s extended information
#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct ThingDefine {
    pub key: String,

    /// For human readable what the `Thing` is.
    pub description: String,

    /// version of the `Thing`
    pub version: u32,

    pub have_states: bool,

    pub states: Option<String>,

    /// Define whats the `Thing` should include
    pub fields: Option<String>,

    pub create_time: DateTime<Utc>,
}


/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InstanceNoID {
    /// This instance's Type
    pub thing: Thing,
    /// The time which plan to flow for this instance
    pub execute_time: u64,
    /// When this instance created
    pub create_time: u64,
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


pub type Result<T> = std::result::Result<T, NatureError>;

pub mod error;


#[cfg(test)]
mod test_instance_without_id;
