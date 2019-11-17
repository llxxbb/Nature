use nature_common::{CONTEXT_TARGET_INSTANCE_ID, ConverterReturned, Instance, NatureError, Protocol, Result};
use nature_db::{InstanceDaoImpl, MetaCacheImpl, MetaDaoImpl, RawTask, TaskDaoImpl};

use crate::controller::{after_converted, process_null, received_self_route};
use crate::task::{ConverterParameterWrapper, TaskForConvert};

pub fn channel_convert(task: TaskForConvert, raw: RawTask) {
    let protocol = task.target.executor.protocol.clone();
    let mut from_instance = task.from.clone();
    // -----begin this logic can't move to place where after converted, because it might not get the last state and cause state conflict
    if protocol == Protocol::Auto {
        let msg = format!("auto converter missed master info, maybe you should fill context with [{}] for meta: {}", CONTEXT_TARGET_INSTANCE_ID, &task.from.meta);
        let err = NatureError::VerifyError(msg);
        let target = from_instance.context.get(CONTEXT_TARGET_INSTANCE_ID);
        let id: Result<String> = match target {
            Some(t) => Ok(t.clone()),
            None => {
                // the master must exists, otherwise `Protocol::Auto` would not be generated.
                let to_meta = task.target.to.clone();
                let master = to_meta.get_setting().unwrap().master.unwrap();
                match master.eq(&from_instance.meta) {
                    true => Ok(from_instance.id.to_string()),
                    false => {
                        let f_meta = MetaCacheImpl::get(&task.from.meta, MetaDaoImpl::get).unwrap();
                        match f_meta.get_setting() {
                            Some(f_setting) => match f_setting.master {
                                None => Err(err.clone()),
                                Some(f_master) => match f_master.eq(&master) {
                                    true => Ok(from_instance.id.to_string()),
                                    false => Err(err.clone()),
                                },
                            }
                            None => Err(err.clone())
                        }
                    }
                }
            }
        };
        match id {
            Ok(id) => {
                from_instance.context.insert(CONTEXT_TARGET_INSTANCE_ID.to_string(), id);
            }
            Err(_) => {
                let _ = TaskDaoImpl::raw_to_error(&err, &raw);
                return;
            }
        }
    }
    // -----end
    let last = match task.target.to.is_state() {
        true => match from_instance.get_last_taget(&task.target.to.meta_string(), InstanceDaoImpl::get_by_id) {
            Err(_) => { return; }
            Ok(last) => last
        }
        false => None
    };
    if Protocol::Auto == protocol {
        let _ = after_converted(&task, &raw, vec![Instance::default()], &last);
        return;
    }
    // init master
    let meta = match MetaCacheImpl::get(&task.from.meta, MetaDaoImpl::get) {
        Ok(m) => m,
        Err(e) => {
            let _ = TaskDaoImpl::raw_to_error(&e, &raw);
            return;
        }
    };
    let master = match task.from.get_master(&meta, InstanceDaoImpl::get_by_id) {
        Ok(m) => m,
        Err(e) => {
            let _ = TaskDaoImpl::raw_to_error(&e, &raw);
            return;
        }
    };
    match ConverterParameterWrapper::gen_and_call_out(&task, raw.task_id.clone(), &task.target, &last, master) {
        Err(err) => match err {
            // only **Environment Error** will be retry
            NatureError::EnvironmentError(_) => (),
            // other error will drop into error
            _ => {
                let _ = TaskDaoImpl::raw_to_error(&err, &raw);
            }
        }
        Ok(returned) => match returned {
            ConverterReturned::Instances(instances) => {
                let _ = after_converted(&task, &raw, instances, &last);
            }
            ConverterReturned::SelfRoute(ins) => {
                let _ = received_self_route(&task, &raw, ins);
            }
            ConverterReturned::Delay(delay) => {
                debug!("delay for meta: {}", meta.meta_string());
                let _ = TaskDaoImpl::update_execute_time(&raw.task_id, i64::from(delay), &last);
            }
            ConverterReturned::LogicalError(ss) => {
                let _ = TaskDaoImpl::raw_to_error(&NatureError::ConverterLogicalError(ss), &raw);
            }
            ConverterReturned::EnvError => (),
            ConverterReturned::None => {
                let _ = process_null(task.target.to.get_meta_type(), &raw.task_id);
            }
        }
    };
}