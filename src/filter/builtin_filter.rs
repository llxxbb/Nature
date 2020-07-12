use std::collections::HashMap;

/// built-in xecutor
use nature_common::{Instance, NatureError, Result};

pub type Filter = fn(para: &mut Instance, cfg: &str) -> Result<()>;


lazy_static! {
    static ref CACHE: HashMap<String,&'static Filter> = init_builtin();
}

fn init_builtin() -> HashMap<String, &'static Filter> {
    info!("BuiltIn filter initialized");
    let mut map: HashMap<String, &'static Filter> = HashMap::new();
    let one: &Filter = &(loader::loader as Filter);
    map.insert("instance-loader".to_string(), one);
    map
}

pub struct BuiltIn;

impl BuiltIn {
    pub fn get(name: &str) -> Result<&'static Filter> {
        match CACHE.get(name) {
            Some(x) => Ok(*x),
            None => Err(NatureError::VerifyError(format!("not exists built-in executor for name : {}", name))),
        }
    }
}


pub mod loader;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_test() {
        assert_eq!(BuiltIn::get("hello").is_err(), true);
        let rtn = BuiltIn::get("instance-loader");
        assert_eq!(rtn.is_ok(), true);
    }
}

