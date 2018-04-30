use data::*;
use global::*;
#[cfg(not(test))]
pub use self::instance_impl::*;
#[cfg(test)]
pub use self::mock::*;
use uuid::UuidBytes;

pub trait InstanceDao {
    fn insert(instance: &Instance) -> Result<()>;
    fn get_last_status_by_id(id: &UuidBytes) -> Result<Option<Instance>>;
    /// check whether source stored earlier
    fn source_stored(instance: &Instance) -> Result<bool>;
}


#[cfg(not(test))]
mod instance_impl;
#[cfg(test)]
mod mock;
