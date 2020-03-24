use nature_common::{ConverterParameter, ConverterReturned, Instance, NatureError, Protocol, Result};
use nature_db::flow_tool::state_check;
use nature_db::Mission;

use crate::task::{HttpExecutorImpl, LocalExecutorImpl, TaskForConvert};

static HTTP_CALLER: &dyn ExecutorTrait = &HttpExecutorImpl;
static LOCAL_RUST_CALLER: &dyn ExecutorTrait = &LocalExecutorImpl;

pub trait ExecutorTrait: Sync {
    fn execute(&self, executor: &str, para: &ConverterParameter) -> ConverterReturned;
}

pub struct ConverterParameterWrapper;

impl ConverterParameterWrapper {
    pub fn gen_and_call_out(task: &TaskForConvert, task_id: Vec<u8>, mission: &Mission, last_target: &Option<Instance>, master: Option<Instance>) -> Result<ConverterReturned>
    {
        if let Some(ref last) = last_target {
            if let Some(demand) = &mission.states_demand {
                if !state_check(&last.states, &demand.need_none, &demand.need_all, &demand.need_any) {
                    return Ok(ConverterReturned::None);
                }
            }
        };

        let para = ConverterParameter {
            from: task.from.clone(),
            last_state: last_target.clone(),
            task_id,
            master,
            cfg: None,
        };
        let executor = Self::get_executer(&mission.executor.protocol)?;
        // debug!("----execute: from: {}, to : {}, executor: {}", task.from.meta, task.target.to.meta_string(), &mission.executor.url);
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
