use data::*;
use global::*;
pub use self::store_plan_impl::*;
use uuid::UuidBytes;

pub trait StorePlanDao {
    /// replace the plan if plan exists.
    fn save(plan: &StorePlan) -> Result<StorePlan>;
    fn get(from_id: &UuidBytes) -> Result<StorePlan>;
}

mod store_plan_impl;