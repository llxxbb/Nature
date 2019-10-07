use nature_common::{ConverterParameter, ConverterReturned, NatureError, Protocol, Result};
use nature_db::Mission;

use crate::task::{HttpExecutorImpl, LocalExecutorImpl, TaskForConvert};

static HTTP_CALLER: &dyn ExecutorTrait = &HttpExecutorImpl;
static LOCAL_RUST_CALLER: &dyn ExecutorTrait = &LocalExecutorImpl;

pub trait ExecutorTrait: Sync {
    fn execute(&self, executor: &str, para: &ConverterParameter) -> ConverterReturned;
}

pub struct ConverterParameterWrapper;

impl ConverterParameterWrapper {
    pub fn gen_and_call_out(task: &TaskForConvert, carrier_id: Vec<u8>, mission: &Mission) -> Result<ConverterReturned> {
        let para = ConverterParameter {
            from: task.from.clone(),
            last_state: task.last_state.clone(),
            carrier_id,
        };
        let executor = Self::get_executer(&mission.executor.protocol)?;
        Ok(executor.execute(&mission.executor.url, &para))
    }

    fn get_executer(protocol: &Protocol) -> Result<&'static dyn ExecutorTrait> {
        match protocol {
            Protocol::Http => Ok(HTTP_CALLER),
            Protocol::LocalRust => Ok(LOCAL_RUST_CALLER),
            _ => Err(NatureError::VerifyError(format!("Did not implement for protocal : {:?}", protocol)))
        }
    }
}
