use std::collections::HashMap;

/// built-in converter
use nature_common::{Convert, NatureError, Result};

lazy_static! {
    static ref CACHE: HashMap<String,Convert> = {
        info!("BuiltIn converter initialized");
        let mut map: HashMap<String,Convert> = HashMap::new();
        map.insert("counter".to_string(), counter::count);
        map
    };
}

pub struct BuiltIn;

impl BuiltIn {
    pub fn get(name: &str) -> Result<Convert> {
        match CACHE.get(name) {
            Some(x) => Ok(x.clone()),
            None => Err(NatureError::VerifyError(format!("not exists built-in converter for name : {}", name))),
        }
    }
}

mod counter;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_test() {
        assert_eq!(BuiltIn::get("hello").is_err(), true);
        assert_eq!(BuiltIn::get("counter").is_ok(), true);
    }
}