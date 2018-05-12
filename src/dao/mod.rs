extern crate r2d2_diesel;

use data::*;
use global::*;
pub use self::carrier::*;
pub use self::mapping::*;
pub use self::orm::*;
pub use self::store_plan::*;
use uuid::UuidBytes;

mod mapping;
mod orm;
mod carrier;
mod store_plan;

pub trait InstanceDao {
    fn insert(instance: &Instance) -> Result<()>;
    fn get_last_status_by_id(id: &UuidBytes) -> Result<Option<Instance>>;
    /// check whether source stored earlier
    fn source_stored(instance: &Instance) -> Result<bool>;
}

pub trait ThingDefineDao {
    fn get(thing: &Thing) -> Result<ThingDefine>;
    fn insert(define: &ThingDefine) -> Result<()>;
}
