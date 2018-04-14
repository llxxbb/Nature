#[cfg(not(test))]
use data::dao::*;
#[cfg(test)]
pub use self::mock::*;
#[cfg(not(test))]
pub use self::store_task_impl::*;
use super::*;

#[cfg(test)]
pub mod mock;
#[cfg(not(test))]
pub mod store_task_impl;