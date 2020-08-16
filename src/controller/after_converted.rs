use std::str::FromStr;

use nature_common::{append_para, CONTEXT_LOOP_FINISHED, CONTEXT_LOOP_ID, CONTEXT_LOOP_NEXT, CONTEXT_LOOP_TASK, Instance, MetaSetting, MetaType, NatureError, Result, SelfRouteInstance};
use nature_db::{D_T, Mission, MissionRaw, RawTask, TaskDao, TaskType};

use crate::controller::{channel_batch, channel_store, get_store_task};
use crate::system::SWITCH_SAVE_DIRECTLY_FOR_ONE;
use crate::task::{Converted, TaskForConvert};

pub async fn after_converted(task: &TaskForConvert, convert_task: &RawTask, instances: Vec<Instance>, last_state: &Option<Instance>) -> Result<()> {
    // debug!("executor returned {} instances for `Meta`: {:?}, from {}", instances.len(), &task.target.to.meta_string(), task.from.get_key());
    match Converted::gen(&task, &convert_task, instances, last_state) {
        Ok(rtn) => {
            // process MetaType::Loop
            let mut rtn = rtn;
            meta_loop(task, &mut rtn)?;
            match rtn.converted.len() {
                0 => match D_T.finish_task(&convert_task.task_id).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        warn!("finish task occur error: {}", e);
                        Err(e)
                    }
                },
                1 => {
                    if state_loop_check(task, &rtn.converted[0], convert_task).await { return Ok(()); }
                    match *SWITCH_SAVE_DIRECTLY_FOR_ONE {
                        true => save_one(rtn, &task.target).await,
                        false => save_batch(rtn).await
                    }
                }
                _ => save_batch(rtn).await
            }
        }
        Err(err) => {
            warn!("pre-process returned instance error:{}, task would be moved to error table", err);
            let _ = D_T.raw_to_error(&err, &convert_task).await;
            Err(err)
        }
    }
}

fn meta_loop(task: &TaskForConvert, rtn: &mut Converted) -> Result<()> {
    if task.target.to.get_meta_type() == MetaType::Loop {
        let setting = match task.target.to.get_setting() {
            Some(set) => set,
            None => {
                let mut set = MetaSetting::default();
                set.output_last = false;
                set
            }
        };
        if let Some(_) = task.from.sys_context.get(CONTEXT_LOOP_FINISHED) {
            // finished need do nothing
            return Ok(());
        }
        if rtn.converted.len() == 1 && setting.output_last {
            // use MetaType::Loop replace the real one
            rtn.converted[0].meta = task.target.to.meta_string();
            gen_instance_for_loop(&mut rtn.converted[0], task)?;
        } else {
            // append a MetaType::Loop instance
            let mut ins = Instance::default();
            ins.meta = task.target.to.meta_string();
            gen_instance_for_loop(&mut ins, task)?;
            rtn.converted.push(ins);
        }
    }
    Ok(())
}

/// **Notice** need get sys_context from upstream
fn gen_instance_for_loop(ins: &mut Instance, task: &TaskForConvert) -> Result<()> {
    ins.id = task.from.id;

    // loop next
    if let Some(next) = task.from.sys_context.get(CONTEXT_LOOP_NEXT) {
        ins.sys_context.insert(CONTEXT_LOOP_NEXT.to_string(), next.to_string());
    }

    // fix ins.para
    fix_loop_id(ins, task)?;

    let raw = MissionRaw::from(task.target.clone()).to_json()?;
    ins.sys_context.insert(CONTEXT_LOOP_TASK.to_string(), raw);
    Ok(())
}

fn fix_loop_id(ins: &mut Instance, task: &TaskForConvert) -> Result<()> {
    if let Some(ver) = task.from.sys_context.get(CONTEXT_LOOP_ID) {
        let mut n_ver = i32::from_str(ver)?;
        let append = if n_ver > 1 {
            n_ver -= 1;
            let mut ver_len = n_ver.to_string().len();
            let para_len = task.from.para.len();
            if para_len > ver_len {
                ver_len += 1;
            }
            let end = para_len as i32 - ver_len as i32;
            if end < 0 {
                return Err(NatureError::SystemError("para of old loop.id has problem".to_string()));
            }
            task.from.para[0..end as usize].to_string()
        } else {
            ins.para.to_string()
        };
        ins.para = append_para(&append, ver);
        ins.sys_context.insert(CONTEXT_LOOP_ID.to_string(), ver.to_string());
    };
    Ok(())
}

async fn state_loop_check(task: &TaskForConvert, ins: &Instance, raw: &RawTask) -> bool {
    if ins.state_version > 0 && ins.state_version == task.conflict_version {
        warn!("looping for conflict: {}, task would be moved to error table", ins.get_key());
        let err = NatureError::LogicalError("conflict looping".to_string());
        let _ = D_T.raw_to_error(&err, &raw).await;
        true
    } else {
        false
    }
}

async fn save_one(converted: Converted, previous_mission: &Mission) -> Result<()> {
    let instance = &converted.converted[0];
    let task = get_store_task(&instance, Some(previous_mission.clone())).await?;
    let rtn = channel_store(task, converted.done_task).await?;
    Ok(rtn)
}

async fn save_batch(converted: Converted) -> Result<()> {
    let raw = RawTask::new(&converted.converted, &converted.done_task.task_key, TaskType::Batch as i8, "")?;
    let num = D_T.insert(&raw).await?;
    let _ = D_T.finish_task(&converted.done_task.task_id).await?;
    if num == 1 {
        let _ = channel_batch(converted.converted, raw).await;
    }
    Ok(())
}

pub async fn process_null(meta_type: MetaType, task_id: &str) -> Result<()> {
    match meta_type {
        MetaType::Null => {
            let _ = D_T.finish_task(task_id).await?;
            Ok(())
        }
        _ => Err(NatureError::VerifyError("need return [ConverterReturned::None]".to_string()))
    }
}

pub fn received_self_route(_task: &TaskForConvert, _raw: &RawTask, _instances: Vec<SelfRouteInstance>) -> Result<()> {
    // TODO unimplemented
    unimplemented!()
}

#[cfg(test)]
mod loop_test {
    use super::*;

    #[test]
    fn nothing() {
        let mut instance = Instance::default();
        let convert = TaskForConvert::default();
        fix_loop_id(&mut instance, &convert).unwrap();
    }

    #[test]
    fn para_only() {
        let mut instance = Instance::default();
        let mut convert = TaskForConvert::default();
        convert.from.para = "llxxbb".to_string();
        fix_loop_id(&mut instance, &convert).unwrap();
        assert_eq!(instance.para, "")
    }

    #[test]
    fn sys_context_1() {
        let mut instance = Instance::default();
        let mut convert = TaskForConvert::default();
        convert.from.sys_context.insert(CONTEXT_LOOP_ID.to_string(), 1.to_string());
        fix_loop_id(&mut instance, &convert).unwrap();
        assert_eq!(instance.para, "1")
    }

    #[test]
    fn sys_context_1_with_other() {
        let mut instance = Instance::default();
        instance.para = "llzzbb".to_string();
        let mut convert = TaskForConvert::default();
        convert.from.sys_context.insert(CONTEXT_LOOP_ID.to_string(), 1.to_string());
        fix_loop_id(&mut instance, &convert).unwrap();
        assert_eq!(instance.para, "llzzbb/1");

        // from has para
        instance.para = "llzzbb".to_string();
        convert.from.para = "made".to_string();
        fix_loop_id(&mut instance, &convert).unwrap();
        assert_eq!(instance.para, "llzzbb/1");
    }

    #[test]
    fn sys_context_10_error() {
        let mut instance = Instance::default();
        let mut convert = TaskForConvert::default();
        convert.from.sys_context.insert(CONTEXT_LOOP_ID.to_string(), 10.to_string());
        let result = fix_loop_id(&mut instance, &convert);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn sys_context_10() {
        let mut instance = Instance::default();
        let mut convert = TaskForConvert::default();
        convert.from.para = "9".to_string();
        convert.from.sys_context.insert(CONTEXT_LOOP_ID.to_string(), 10.to_string());
        fix_loop_id(&mut instance, &convert).unwrap();
        assert_eq!(instance.para, "10")
    }

    #[test]
    fn sys_context_10_with_other() {
        let mut instance = Instance::default();
        let mut convert = TaskForConvert::default();
        convert.from.para = "llxxbb/9".to_string();
        convert.from.sys_context.insert(CONTEXT_LOOP_ID.to_string(), 10.to_string());
        fix_loop_id(&mut instance, &convert).unwrap();
        assert_eq!(instance.para, "llxxbb/10")
    }
}