
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
/// `DateTime` is not implement `Default` trait
#[derive(Serialize, Deserialize, Debug, Queryable, Default, Clone)]
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

    pub create_time: u64,
}
