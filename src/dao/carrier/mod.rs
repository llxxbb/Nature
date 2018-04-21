use data::*;
use global::*;
#[cfg(not(test))]
pub use self::carrier_impl::*;
#[cfg(test)]
pub use self::mock::*;
use serde::Serialize;
use uuid::*;
use task::*;

pub trait CarrierDao {
    fn insert<T: Sized + Serialize>(carrier: &Carrier<T>) -> Result<UuidBytes>;
    fn delete(id: &UuidBytes) -> Result<()>;
    fn move_to_error<T: Sized + Serialize>(err : CarryError<T>);
}


pub mod carrier_impl;

#[cfg(test)]
pub mod mock;