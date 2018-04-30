use data::*;
pub use self::plan_impl::*;
use uuid::UuidBytes;

/// **unique key**
/// * from_id
/// * from_thing
#[derive(Debug)]
pub struct StorePlan {
    pub from_id: UuidBytes,
    pub from_thing: Thing,
    pub to: Thing,
    pub plan: Vec<Instance>,
}

mod plan_impl;