use std::panic::catch_unwind;

use nature_common::{ConverterParameter, ConverterReturned, Instance, NatureError, Protocol};
use nature_db::{Mission, RawTask};
use nature_db::flow_tool::state_check;

use crate::task::{http_execute_async, TaskForConvert};
use crate::task::filter::filter;
use crate::task::local_common::local_execute;
use crate::builtin_executor::BuiltIn;

pub type Execute = fn(para: &ConverterParameter) -> ConverterReturned;

pub async fn gen_and_call_out(task: &TaskForConvert, raw: &RawTask, mission: &Mission, last_target: &Option<Instance>, master: Option<Instance>) -> ConverterReturned {
    if let Some(ref last) = last_target {
        if let Some(demand) = &mission.target_demand.states {
            if !state_check(&last.states, &demand.need_none, &demand.need_all, &demand.need_any) {
                return ConverterReturned::None;
            }
        }
    };
    let mut from = task.from.clone();
    match filter(&mut from, &task.target.filter_before).await {
        Err(NatureError::EnvironmentError(e)) => return ConverterReturned::EnvError(e),
        Err(e) => return ConverterReturned::LogicalError(e.to_string()),
        _ => ()
    };
    let para = ConverterParameter {
        from,
        last_state: last_target.clone(),
        task_id: raw.task_id.clone(),
        master,
        cfg: mission.executor.settings.to_string(),
    };
    debug!("execute: from: {}, to : {}, executor: {}", task.from.meta, task.target.to.meta_string(), &mission.executor.url);
    let rtn = match &mission.executor.protocol {
        Protocol::Http => http_execute_async(&mission.executor.url, &para).await,
        Protocol::LocalRust => match local_execute(&mission.executor.url, &para).await {
            Ok(rtn) => rtn,
            Err(err) => ConverterReturned::EnvError(err.to_string())
        }
        Protocol::BuiltIn => match BuiltIn::get(&mission.executor.url) {
            Ok(exe) => {
                match catch_unwind(|| { exe(&para) }) {
                    Ok(rtn) => {
                        rtn
                    }
                    Err(e) => {
                        warn!("{:?} return error: {:?}", mission.executor.url, e);
                        ConverterReturned::LogicalError("executor implement error".to_string())
                    }
                }
            }
            Err(_) => ConverterReturned::LogicalError("get built-in executor failed".to_string())
        }
        _ => ConverterReturned::LogicalError(format!("Did not implement for protocal : {:?}", &mission.executor.protocol)),
    };
    rtn
}