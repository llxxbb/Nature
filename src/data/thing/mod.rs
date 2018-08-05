use chrono::prelude::*;
use global::*;
use nature_common::*;


//trace_macros!(true);

/// `Thing`'s extended information
/// `DateTime` is not implement `Default` trait
#[derive(Serialize, Deserialize, Debug, Queryable, Clone, PartialOrd, PartialEq)]
pub struct ThingDefine {
    pub key: String,

    /// For human readable what the `Thing` is.
    pub description: Option<String>,

    /// version of the `Thing`
    pub version: i32,

    pub states: Option<String>,

    /// Define whats the `Thing` should include
    pub fields: Option<String>,

    pub create_time: NaiveDateTime,
}

//trace_macros!(false);

impl Default for ThingDefine {
    fn default() -> Self {
        ThingDefine {
            key: String::new(),
            description: None,
            version: 0,
            states: None,
            fields: None,
            create_time: Local::now().naive_local(),
        }
    }
}

impl ThingDefine {
    pub fn is_status(&self) -> bool {
        !self.states.is_none()
    }
}

#[cfg(test)]
mod test;