use data::thing::*;
use global::*;
#[cfg(test)]
pub use self::mock::*;
#[cfg(not(test))]
pub use self::thing_impl::*;

pub trait ThingDefineDao {
    fn get(thing: &Thing) -> Result<ThingDefine>;
}


mod thing_impl;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;