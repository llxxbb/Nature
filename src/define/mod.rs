pub use self::error::NatureError;
pub use self::instance::Instance;
use std;
pub use store::Store;
use uuid::UuidBytes;


///! A public lib for outer user call


/// `Thing`'s basic information
#[derive(Serialize, Deserialize, Debug, Default)]
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
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ThingExtended {
    pub thing: Thing,

    /// For human readable what the `Thing` is.
    pub name: String,

    /// Define whats the `Thing` should include
    pub define: String,
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

pub trait Nature {
    fn flow(&self, thing: Instance) -> Result<UuidBytes>;
//    fn input_batch(&self, batch: Vec<WorldConnectionInput>) -> Result<u64, String>;
//    fn converter_callback(&self) -> Result<u64, String>;
//    fn query(&self);
}

pub mod instance;
pub mod error;


#[cfg(test)]
mod tests;
