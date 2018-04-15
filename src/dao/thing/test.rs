use data::*;
use super::thing_impl::*;
use super::ThingDefineDao;

#[test]
fn get_from_db() {
    // TODO
    let thing = Thing::default();
    let rtn = ThingDefineDaoService::get(&thing);
    assert!(rtn.is_err());
}

#[test]
fn get_from_cache() {
    // TODO
}