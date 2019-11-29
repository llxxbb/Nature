use std::collections::HashMap;

/// built-in converter
use nature_common::{NatureError, Result};

use crate::task::ExecutorTrait;

lazy_static! {
    static ref CACHE: HashMap<String,&'static dyn ExecutorTrait> = {
        info!("BuiltIn converter initialized");
        let mut map: HashMap<String,&'static dyn ExecutorTrait> = HashMap::new();
        let cnt : &dyn ExecutorTrait = &counter::Counter{};
        map.insert("counter".to_string(), cnt);
        map
    };
}

pub struct BuiltIn;

impl BuiltIn {
    pub fn get(name: &str) -> Result<&'static dyn ExecutorTrait> {
        match CACHE.get(name) {
            Some(x) => Ok(*x),
            None => Err(NatureError::VerifyError(format!("not exists built-in converter for name : {}", name))),
        }
    }
}

mod counter;

#[cfg(test)]
mod test {
    use super::*;
    use nature_common::{ConverterParameter, ConverterReturned};

    #[test]
    fn get_test() {
        assert_eq!(BuiltIn::get("hello").is_err(), true);
        let rtn = BuiltIn::get("counter");
        assert_eq!(rtn.is_ok(), true);
    }
}