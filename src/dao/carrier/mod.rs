use data::carrier::*;
use global::*;
#[cfg(not(test))]
pub use self::carrier_impl::*;
#[cfg(test)]
pub use self::mock::*;
use uuid::*;

pub trait CarrierDao {
    fn insert<T>(carrier: &Carrier<T>) -> Result<UuidBytes>;
}


pub mod carrier_impl;

#[cfg(test)]
pub mod mock;