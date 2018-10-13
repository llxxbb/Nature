use flow::convert::local::LocalExecutorTrait;
use nature_common::*;
use nature_db::*;
use std::rc::Rc;

pub trait CallOutTrait {
    fn convert(&self, mission: &Mission, para: &CallOutParameter) -> Result<ConverterReturned>;
}

pub struct CallOutImpl {
    pub local_rust: Rc<LocalExecutorTrait>,
}

impl CallOutTrait for CallOutImpl {
    fn convert(&self, mission: &Mission, para: &CallOutParameter) -> Result<ConverterReturned> {
        match mission.executor.protocol {
            Protocol::LocalRust => {
                Ok(self.local_rust.execute(&mission.executor.url, para))
            }
            _ => {
                // TODO
                unimplemented!()
            }
        }
    }
}
