use data::*;
use global::*;
#[cfg(test)]
pub use self::mock::*;
#[cfg(not(test))]
pub use self::thing_impl::*;

pub trait ThingDefineService {
    fn get(thing: &Thing) -> Result<ThingDefine>;
    fn insert(define : &ThingDefine) -> Result<()>;
}


mod thing_impl;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;