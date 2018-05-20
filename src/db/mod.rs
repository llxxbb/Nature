use data::*;
use global::*;
#[cfg(not(test))]
pub use self::cache::*;
pub use self::dao_impl::*;
#[cfg(test)]
pub use self::mock::*;
#[cfg(not(test))]
pub use self::orm::*;
#[cfg(not(test))]
pub use self::sqlite::*;
use uuid::UuidBytes;

// open this cause test unable to run
//#[cfg(not(test))]
mod sqlite;
mod cache;
mod dao_impl;
mod orm;
#[cfg(test)]
mod mock;
