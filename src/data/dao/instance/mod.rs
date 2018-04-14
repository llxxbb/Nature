use data::instance::*;
use define::*;
#[cfg(not(test))]
pub use self::instance_impl::*;
#[cfg(test)]
pub use self::mock::*;
use uuid::UuidBytes;


pub trait InstanceDao {
    fn insert(instance: &Instance) -> Result<UuidBytes>;
}


#[cfg(not(test))]
mod instance_impl;
#[cfg(test)]
mod mock;
