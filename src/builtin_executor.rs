use std::collections::HashMap;

/// built-in xecutor
use nature_common::{NatureError, Result};

use dimension_splitter::dimension_split;

use crate::task::Execute;

lazy_static! {
    static ref CACHE: HashMap<String,&'static Execute> = {
        info!("BuiltIn executor initialized");
        let mut map: HashMap<String,&'static Execute> = HashMap::new();
        let cnt : &Execute = &(dimension_split as Execute);
        map.insert("dimensionSplit".to_string(), cnt);
        map
    };
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

mod dimension_splitter;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_test() {
        assert_eq!(BuiltIn::get("hello").is_err(), true);
        let rtn = BuiltIn::get("simpleCounter");
        assert_eq!(rtn.is_ok(), true);
    }
}