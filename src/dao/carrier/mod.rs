use data::*;
use global::*;
#[cfg(not(test))]
pub use self::carrier_impl::*;
#[cfg(test)]
pub use self::mock::*;
use serde::Serialize;
use task::*;
use uuid::*;

pub trait CarrierDao {
    fn insert<T: Sized + Serialize>(carrier: &Carrier<T>) -> Result<UuidBytes>;
    fn delete(id: &UuidBytes) -> Result<()>;
    fn move_to_error<T: Sized + Serialize>(err: CarryError<T>) -> Result<()>;
    fn update_execute_time(_id: UuidBytes, new_time: i64) -> Result<()>;
    fn get<T: Sized + Serialize>(_id: UuidBytes) -> Result<Carrier<T>>;
}

pub mod carrier_impl;

#[cfg(test)]
pub mod mock;