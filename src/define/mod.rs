use uuid::UuidBytes;

///! A public lib for outer user call
///
///


/// `Thing`'s basic information
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Thing {
    /// To identify a `Thing`.
    /// A `Thing` may have a lots of `ThingInstance`s, so it's a **Class** for ThingInstance`.
    /// Because there are huge quantity of `Thing`s , so we need a way to organize `Thing`s.
    /// A way is to set name with hierarchical structures,
    ///
    /// # Example
    ///
    /// ```
    ///     "shop/order"
    /// ```
    pub id: String,


    /// A `Thing` can be changed in future, the `version` will support this without effect the old ones
    pub version: u32,
}

/// `Thing`'s extended information
pub struct ThingExtended{

    pub thing: Thing,

    /// For human readable what the `Thing` is.
    pub name : String,

    /// Define whats the `Thing` should include
    pub define: String,
}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ThingInstance {
    pub thing: Thing,
    pub instance_id: UuidBytes,
    pub execute_time:u64,
    pub operate_time:u64,
    pub content: String,
    pub context: String,
}

pub trait Nature {
    fn transform(&self, instance: ThingInstance) -> Result<UuidBytes, String>;
//    fn input_batch(&self, batch: Vec<WorldConnectionInput>) -> Result<u64, String>;
//    fn converter_callback(&self) -> Result<u64, String>;
//    fn query(&self);
}

