use std::collections::HashMap;

use dimension_splitter::dimension_split;
/// built-in xecutor
use nature_common::{NatureError, Result};
use sum::sum;
use sum_allow_repeat::sum_allow_repeat;

use crate::task::Execute;

lazy_static! {
    static ref CACHE: HashMap<String,&'static Execute> = init_builtin();
}

fn init_builtin() -> HashMap<String, &'static Execute> {
    info!("BuiltIn executor initialized");
    let mut map: HashMap<String, &'static Execute> = HashMap::new();
    let one: &Execute = &(dimension_split as Execute);
    map.insert("dimensionSplit".to_string(), one);
    let one: &Execute = &(sum as Execute);
    map.insert("sum".to_string(), one);
    let one: &Execute = &(sum_allow_repeat as Execute);
    map.insert("sum_allow_repeat".to_string(), one);
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

mod dimension_splitter;
mod sum;
mod sum_allow_repeat;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_test() {
        assert_eq!(BuiltIn::get("hello").is_err(), true);
        let rtn = BuiltIn::get("dimensionSplit");
        assert_eq!(rtn.is_ok(), true);
    }
}