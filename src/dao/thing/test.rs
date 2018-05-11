use data::*;
use super::thing_impl::*;
use super::ThingDefineService;

#[test]
fn get_from_db() {
    // TODO
    let thing = Thing::default();
    let rtn = ThingDefineServiceImpl::get(&thing);
    assert!(rtn.is_err());
}

#[test]
fn get_from_cache() {
    // TODO
}