use data::*;
use global::*;
#[cfg(not(test))]
pub use self::carrier_impl::*;
#[cfg(test)]
pub use self::mock::*;
use serde::Serialize;
use uuid::*;

pub trait CarrierDao {
    fn insert<T: Sized + Serialize>(carrier: &Carrier<T>) -> Result<UuidBytes>;
    fn delete(id: &UuidBytes) -> Result<()>;
}


pub mod carrier_impl;

#[cfg(test)]
pub mod mock;