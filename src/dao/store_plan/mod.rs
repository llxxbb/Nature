use data::*;
use global::*;
pub use self::store_plan_impl::*;

pub trait StorePlanDao {
    /// replace the plan if plan exists.
    fn save(plan: &mut StorePlan) -> Result<()>;
}

mod store_plan_impl;