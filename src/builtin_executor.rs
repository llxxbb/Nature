use std::collections::HashMap;

/// built-in xecutor
use nature_common::{NatureError, Result};
use scatter::scatter;
use sum::sum;
use time_range::time_range;

use crate::task::Execute;

lazy_static! {
    static ref CACHE: HashMap<String,&'static Execute> = init_builtin();
}

fn init_builtin() -> HashMap<String, &'static Execute> {
    info!("BuiltIn executor initialized");
    let mut map: HashMap<String, &'static Execute> = HashMap::new();
    let one: &Execute = &(scatter as Execute);
    map.insert("scatter".to_string(), one);
    let one: &Execute = &(sum as Execute);
    map.insert("sum".to_string(), one);
    let one: &Execute = &(time_range as Execute);
    map.insert("time_range".to_string(), one);
    map
}

pub struct BuiltIn;

impl BuiltIn {
    pub fn get(name: &str) -> Result<&'static Execute> {
        match CACHE.get(name) {
            Some(x) => Ok(*x),
            None => Err(NatureError::VerifyError(format!("not exists built-in executor for name : {}", name))),
        }
    }
}

// mod dimension_splitter;
mod scatter;
mod sum;
mod time_range;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_test() {
        assert_eq!(BuiltIn::get("hello").is_err(), true);
        let rtn = BuiltIn::get("scatter");
        assert_eq!(rtn.is_ok(), true);
    }
}