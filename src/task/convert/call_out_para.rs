use nature_common::{ConverterParameter, ConverterReturned, Instance, NatureError, Protocol, Result};
use nature_db::Mission;

use crate::task::{HttpExecutorImpl, LocalExecutorImpl, TaskForConvert};

static HTTP_CALLER: &dyn ExecutorTrait = &HttpExecutorImpl;
static LOCAL_RUST_CALLER: &dyn ExecutorTrait = &LocalExecutorImpl;

pub trait ExecutorTrait: Sync {
    fn execute(&self, executor: &str, para: &ConverterParameter) -> ConverterReturned;
}

pub struct ConverterParameterWrapper;

impl ConverterParameterWrapper {
    pub fn gen_and_call_out(task: &TaskForConvert, carrier_id: Vec<u8>, mission: &Mission, last_target: &Option<Instance>, master: Option<Instance>) -> Result<ConverterReturned>
    {
        if let Some(ref last) = last_target {
            if let Some(demand) = &mission.states_demand {
                if let Err(_) = demand.check_last(&last.states) {
                    return Ok(ConverterReturned::None);
                }
            }
        };

        let para = ConverterParameter {
            from: task.from.clone(),
            last_state: last_target.clone(),
            carrier_id,
            master,
        };

        let executor = Self::get_executer(&mission.executor.protocol)?;
        let rtn = executor.execute(&mission.executor.url, &para);
        Ok(rtn)
    }

    fn get_executer(protocol: &Protocol) -> Result<&'static dyn ExecutorTrait> {
        match protocol {
            Protocol::Http => Ok(HTTP_CALLER),
            Protocol::LocalRust => Ok(LOCAL_RUST_CALLER),
            _ => Err(NatureError::VerifyError(format!("Did not implement for protocal : {:?}", protocol)))
        }
    }
}
