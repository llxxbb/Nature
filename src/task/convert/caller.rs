use nature_common::*;
use nature_db::*;

use crate::task::{HttpExecutorImpl, LocalExecutorImpl};

pub trait ExecutorTrait: Sync {
    fn execute(&self, executor: &str, para: &CallOutParameter) -> ConverterReturned;
}

trait CallOutTrait {
    fn convert(&self, mission: &Mission, para: &CallOutParameter) -> Result<ConverterReturned>;
}

static HTTP_CALLER: &dyn ExecutorTrait = &HttpExecutorImpl;
static LOCAL_RUST_CALLER: &dyn ExecutorTrait = &LocalExecutorImpl;

pub struct CallerService;

impl CallerService {
    pub fn convert(mission: &Mission, para: &CallOutParameter) -> Result<ConverterReturned> {
        let executer = Self::get_executer(&mission.executor.protocol)?;
        Ok(executer.execute(&mission.executor.url, para))
    }

    fn get_executer(protocol: &Protocol) -> Result<&'static dyn ExecutorTrait> {
        match protocol {
            Protocol::Http => Ok(HTTP_CALLER),
            Protocol::LocalRust => Ok(LOCAL_RUST_CALLER),
            _ => Err(NatureError::ConverterProtocalError(format!("Did not implement for protocal : {:?}", protocol)))
        }
    }
}