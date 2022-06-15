use actix_rt::Runtime;

use crate::db::{C_M, D_M, D_T, InstanceDaoImpl, MetaCache, RawTask, TaskDao};
use crate::domain::*;
use crate::nature_lib::dispatcher::{after_converted, received_self_route};
use crate::nature_lib::middleware::filter::filter_result;
use crate::nature_lib::task::{call_executor, TaskForConvert};

/// **Notice**: Can't use async under actix-rt directly, otherwise it can lead to "actix-rt overflow its stack".
/// So changed it to traditional mpsc
pub fn channel_convert(store: (TaskForConvert, RawTask)) {
    let runtime = match Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            warn!("get tokio runtime error : {}", e.to_string());
            return;
        }
    };
    runtime.block_on(do_convert(store.0, store.1));
}

pub(crate) async fn do_convert(task: TaskForConvert, raw: RawTask) {
    // debug!("---task for convert: from:{}, to {}", task.from.meta, task.target.to.meta_string());
    let protocol = task.target.executor.protocol.clone();
    let mut from_instance = task.from.clone();
    let mut task = task;
    // -----begin this logic can't move to place where after converted, because it might not get the last state and cause state conflict
    if protocol == Protocol::Auto {
        init_target_id_for_sys_context(&mut task, &mut from_instance).await
    }
    // -----end
    let mut task = task;
    let last = match InstanceDaoImpl::select_last_target(&from_instance, &mut task.target).await {
        Err(_) => { return; }
        Ok(last) => last
    };
    if Protocol::Auto == protocol {
        let _ = after_converted(&task, &raw, vec![Instance::default()], &last).await;
        return;
    }
    // init master
    let meta = match C_M.get(&task.from.path.meta, &*D_M).await {
        Ok(m) => m,
        Err(e) => {
            warn!("get meta error: {}", e);
            let _ = D_T.raw_to_error(&e, &raw).await;
            return;
        }
    };
    let master = match task.from.get_master(&meta, InstanceDaoImpl::select_by_id).await {
        Ok(m) => m,
        Err(e) => {
            warn!("get master instance error: {}", e);
            let _ = D_T.raw_to_error(&e, &raw).await;
            return;
        }
    };
    let rtn = call_executor(&mut task, &raw, &last, master).await;
    match handle_converted(rtn, &task, &raw, &last).await {
        Ok(()) => (),
        Err(NatureError::EnvironmentError(_)) => (),
        Err(e) => {
            warn!("call out error: {}", e);
            let _ = D_T.raw_to_error(&e, &raw).await;
        }
    }
}

async fn handle_converted(converted: ConverterReturned, task: &TaskForConvert, raw: &RawTask, last: &Option<Instance>) -> Result<()> {
    match converted {
        ConverterReturned::Instances { ins: mut instances } => {
            filter_result(&mut instances, &task.target.convert_after).await?;
            after_converted(task, &raw, instances, &last).await?;
        }
        ConverterReturned::SelfRoute { ins } => {
            let _ = received_self_route(task, &raw, ins);
        }
        ConverterReturned::Delay { num: delay } => {
            debug!("delay task from meta: {}", task.from.path.meta);
            let _ = D_T.update_execute_time(&raw.task_id, i64::from(delay)).await;
        }
        ConverterReturned::LogicalError { msg: ss } => {
            warn!("executor returned logic err from : {}, task would be deleted", task.from.path.meta);
            let _ = D_T.raw_to_error(&NatureError::LogicalError(ss), &raw).await;
        }
        ConverterReturned::EnvError { msg: e } => {
            warn!("executor returned env err: {}", e);
        }
        ConverterReturned::None => {
            let mut ins = Instance::default();
            ins.path.meta = "N::1".to_string();
            after_converted(task, &raw, vec![ins], &last).await?;
        }
    }
    Ok(())
}

async fn init_target_id_for_sys_context(task: &mut TaskForConvert, from_instance: &mut Instance) -> () {
    let target = task.target.sys_context.get(CONTEXT_TARGET_INSTANCE_ID);
    if let Some(_) = target {
        return;
    }
    let setting = task.target.to.get_setting();
    if setting.is_none() {
        return;
    }
    let setting = setting.unwrap();
    if setting.master.is_none() {
        return;
    }
    let master = setting.master.unwrap();
    if master.eq(&from_instance.path.meta) {
        task.target.sys_context.insert(CONTEXT_TARGET_INSTANCE_ID.to_string(), from_instance.id.to_string());
        return;
    }
    let f_meta: Meta = C_M.get(&task.from.path.meta, &*D_M).await.unwrap();
    if f_meta.get_setting().is_none() {
        return;
    }
    let f_setting = f_meta.get_setting().unwrap();
    if f_setting.master.is_none() {
        return;
    }
    let f_master = f_setting.master.unwrap();
    if f_master.eq(&master) {
        task.target.sys_context.insert(CONTEXT_TARGET_INSTANCE_ID.to_string(), format!("{}", from_instance.id));
    }
}


