use actix_rt::Runtime;

use nature_common::{CONTEXT_TARGET_INSTANCE_ID, ConverterReturned, Instance, NatureError, Protocol, Result};
use nature_db::{InstanceDaoImpl, MetaCacheImpl, MG, Mission, RawTask, TaskDaoImpl};

use crate::controller::{after_converted, process_null, received_self_route};
use crate::task::{gen_and_call_out, TaskForConvert};
use crate::task::filter::filter_instance;

/// **Notice**: Can't use async under actix-rt directly, otherwise it can lead to "actix-rt overflow its stack".
/// So changed it to traditional mpsc
pub fn channel_convert(store: (TaskForConvert, RawTask)) {
    let mut runtime = match Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            warn!("get tokio runtime error : {}", e.to_string());
            return;
        }
    };
    runtime.block_on(do_convert(store.0, store.1));
}

async fn do_convert(task: TaskForConvert, raw: RawTask) {
    // debug!("---task for convert: from:{}, to {}", task.from.meta, task.target.to.meta_string());
    let protocol = task.target.executor.protocol.clone();
    let mut from_instance = task.from.clone();
    // -----begin this logic can't move to place where after converted, because it might not get the last state and cause state conflict
    if protocol == Protocol::Auto {
        init_target_id_for_sys_context(&task, &raw, &mut from_instance)
    }
    // -----end
    let last = match InstanceDaoImpl::get_last_taget(&from_instance, &task.target) {
        Err(_) => { return; }
        Ok(last) => last
    };
    if Protocol::Auto == protocol {
        let _ = after_converted(&task, &raw, vec![Instance::default()], &last).await;
        return;
    }
    // init master
    let meta = match MetaCacheImpl::get(&task.from.meta, MG) {
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
    let rtn = gen_and_call_out(&task, &raw, &task.target, &last, master).await;
    let _ = handle_converted(rtn, &task, &raw, &task.target, &last).await;
}

async fn handle_converted(converted: ConverterReturned, task: &TaskForConvert, raw: &RawTask, mission: &Mission, last: &Option<Instance>) -> Result<()> {
    match converted {
        ConverterReturned::Instances(mut instances) => {
            if instances.len() > 0 && task.target.filter.len() > 0 {
                filter_instance(&mut instances, &task.target.filter).await?;
            }
            let _ = after_converted(task, &raw, instances, &last).await;
        }
        ConverterReturned::SelfRoute(ins) => {
            let _ = received_self_route(task, &raw, ins);
        }
        ConverterReturned::Delay(delay) => {
            debug!("delay task from meta: {}", task.from.meta);
            let _ = TaskDaoImpl::update_execute_time(&raw.task_id, i64::from(delay));
        }
        ConverterReturned::LogicalError(ss) => {
            warn!("executor returned logic err from : {}, task would be deleted", task.from.meta);
            let _ = TaskDaoImpl::raw_to_error(&NatureError::LogicalError(ss), &raw);
        }
        ConverterReturned::EnvError(e) => {
            warn!("executor returned env err: {}", e);
        }
        ConverterReturned::None => {
            let _ = process_null(mission.to.get_meta_type(), &raw.task_id);
        }
    }
    Ok(())
}

fn init_target_id_for_sys_context(task: &TaskForConvert, raw: &RawTask, from_instance: &mut Instance) -> () {
    let msg = r#"Auto Executor need statisfy any of the following conditions:
 exists from_instance.sys_context:target.id
 to.meta.master == from.meta
 to.meta.master == from.meta.master
    "#;
    let target = from_instance.sys_context.get(CONTEXT_TARGET_INSTANCE_ID);
    let f_meta = MetaCacheImpl::get(&task.from.meta, MG).unwrap();
    let to_meta = task.target.to.clone();
    let msg = format!("relation defined error {} to {} . {}", f_meta.meta_string(), to_meta.meta_string(), msg);
    let err = NatureError::VerifyError(msg.to_string());
    let id: Result<String> = match target {
        Some(t) => Ok(t.clone()),
        None => {
// the master must exists, otherwise `Protocol::Auto` would not be generated.
            let master = to_meta.get_setting().unwrap().master.unwrap();
            match master.eq(&from_instance.meta) {
                true => Ok(from_instance.id.to_string()),
                false => {
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
            from_instance.sys_context.insert(CONTEXT_TARGET_INSTANCE_ID.to_string(), id);
        }
        Err(_) => {
            warn!("{}", msg);
            let _ = TaskDaoImpl::raw_to_error(&err, &raw);
        }
    }
}