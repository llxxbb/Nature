use std::collections::HashMap;
use std::sync::Arc;

use loader::Loader;
/// built-in xecutor
use para_as_key::ParaAsKey;
use task_checker::TaskCheckerFilter;

use crate::domain::*;
use crate::util::system::INS_KEY_GT;

#[async_trait]
pub trait FilterBefore: Sync + Send {
    async fn filter(&self, ins: &mut Instance, cfg: &str) -> Result<()>;
}

lazy_static! {
    static ref CACHE: HashMap<String, Arc<dyn FilterBefore>> = init_builtin();
}

fn init_builtin() -> HashMap<String, Arc<dyn FilterBefore>> {
    info!("BuiltIn filter initialized");
    let mut map: HashMap<String, Arc<dyn FilterBefore>> = HashMap::new();
    let one = Loader { dao: INS_KEY_GT.clone() };
    map.insert("instance-loader".to_string(), Arc::new(one));
    let one = TaskCheckerFilter {};
    map.insert("task-checker".to_string(), Arc::new(one));
    let one = ParaAsKey {};
    map.insert("para_as_key".to_string(), Arc::new(one));
    map
}

pub struct BuiltIn;

impl BuiltIn {
    pub fn get(name: &str) -> Result<Arc<dyn FilterBefore>> {
        match CACHE.get(name) {
            Some(x) => Ok(x.clone()),
            None => Err(NatureError::VerifyError(format!("not exists built-in executor for name : {}", name))),
        }
    }
}


pub mod loader;
pub mod task_checker;
pub mod para_as_key;


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

