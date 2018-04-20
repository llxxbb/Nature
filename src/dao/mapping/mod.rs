use data::*;
use global::*;
#[cfg(not(test))]
pub use self::dao_impl::*;
#[cfg(test)]
pub use self::mock::*;
use task::RouteInfo;

pub trait MappingDao {
    fn get_relations(instance: &Instance) -> Result<Option<RouteInfo>>;
}

pub mod dao_impl;
#[cfg(test)]
pub mod mock;