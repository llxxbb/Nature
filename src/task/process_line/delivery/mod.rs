#[cfg(not(test))]
pub use self::delivery_impl::*;
#[cfg(test)]
pub use self::mock::*;
use super::*;

mod delivery_impl;


pub trait DeliveryTrait {
    fn create_carrier<T>(valuable: T) -> Result<Carrier<T>> where T: Sized + Serialize;
    fn create_and_finish_carrier<T, U>(valuable: T, old: Carrier<U>) -> Result<Carrier<T>> where T: Sized + Serialize, U: Sized + Serialize;
    fn create_batch_and_finish_carrier<T, U>(valuables: Vec<T>, old: Carrier<U>) -> Result<Vec<Carrier<T>>> where T: Sized + Serialize, U: Sized + Serialize;
    fn finish_carrier(id: &UuidBytes) -> Result<()>;
    fn move_to_err<T>(err: NatureError, carrier: Carrier<T>) where T: Sized + Serialize;
}

#[cfg(test)]
mod mock;
