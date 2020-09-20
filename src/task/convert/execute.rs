use std::panic::catch_unwind;

use crate::builtin_converter::BuiltIn;
use crate::common::{ConverterParameter, ConverterReturned, Instance, NatureError, Protocol};
use crate::db::flow_tool::state_check;
use crate::db::RawTask;
use crate::filter::convert_before;
use crate::task::{http_execute_async, TaskForConvert};
use crate::task::local_common::local_execute;

pub type Execute = fn(para: &ConverterParameter) -> ConverterReturned;

pub async fn call_executor(task: &mut TaskForConvert, raw: &RawTask, last_target: &Option<Instance>, master: Option<Instance>) -> ConverterReturned {
    if let Some(ref last) = last_target {
        if let Some(demand) = &task.target.target_demand.states {
            if !state_check(&last.states, &demand.need_none, &demand.need_all, &demand.need_any) {
                return ConverterReturned::None;
            }
        }
    };
    &task.from;
    match convert_before(&mut task.from, task.target.convert_before.clone()).await {
        Err(NatureError::EnvironmentError(e)) => return ConverterReturned::EnvError(e),
        Err(e) => return ConverterReturned::LogicalError(e.to_string()),
        _ => ()
    };
    let para = ConverterParameter {
        from: task.from.clone(),
        last_state: last_target.clone(),
        task_id: raw.task_id.clone(),
        master,
        cfg: task.target.executor.settings.to_string(),
    };
    debug!("execute: from: {}, to : {}, executor: {}", task.from.meta, task.target.to.meta_string(), &task.target.executor.url);
    let rtn = match &task.target.executor.protocol {
        Protocol::Http => http_execute_async(&task.target.executor.url, &para).await,
        Protocol::LocalRust => match local_execute(&task.target.executor.url, &para).await {
            Ok(rtn) => rtn,
            Err(err) => ConverterReturned::EnvError(err.to_string())
        }
        Protocol::BuiltIn => match BuiltIn::get(&task.target.executor.url) {
            Ok(exe) => {
                match catch_unwind(|| { exe(&para) }) {
                    Ok(rtn) => {
                        rtn
                    }
                    Err(e) => {
                        warn!("{:?} return error: {:?}", task.target.executor.url, e);
                        ConverterReturned::LogicalError("executor implement error".to_string())
                    }
                }
            }
            Err(_) => ConverterReturned::LogicalError("get built-in executor failed".to_string())
        }
        _ => ConverterReturned::LogicalError(format!("Did not implement for protocal : {:?}", &task.target.executor.protocol)),
    };
    rtn
}