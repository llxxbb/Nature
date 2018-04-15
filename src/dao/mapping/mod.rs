use data::*;
#[cfg(not(test))]
pub use self::dao_impl::*;
#[cfg(test)]
pub use self::mock::*;

pub trait MappingDao {
    fn get_relations(instance: &Instance) -> Vec<Mapping>;
}

pub mod dao_impl;
#[cfg(test)]
pub mod mock;