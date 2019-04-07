use std::rc::Rc;

use nature_common::*;
use nature_db::*;

pub trait ExecutorTrait {
    fn execute(&self, executor: &str, para: &CallOutParameter) -> ConverterReturned;
}


pub trait CallOutTrait {
    fn convert(&self, mission: &Mission, para: &CallOutParameter) -> Result<ConverterReturned>;
}

pub struct CallOutImpl {
    pub local_rust: Rc<ExecutorTrait>,
    pub http_caller:Rc<ExecutorTrait>,
}

impl CallOutTrait for CallOutImpl {
    fn convert(&self, mission: &Mission, para: &CallOutParameter) -> Result<ConverterReturned> {
        match mission.executor.protocol {
            Protocol::LocalRust => {
                debug!("  call local converter");
                Ok(self.local_rust.execute(&mission.executor.url, para))
            }
            Protocol::Http => {
                Ok(self.http_caller.execute(&mission.executor.url, para))
            }
            _ => {
                Err(NatureError::ConverterProtocalError(format!("Did not implement for protocal : {:?}", mission.executor.protocol)))
            }
        }
    }
}
