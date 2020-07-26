use nature_common::{CONTEXT_LOOP_FINISHED, CONTEXT_LOOP_NEXT, CONTEXT_TARGET_INSTANCE_ID, CONTEXT_TARGET_INSTANCE_PARA, FromInstance, get_para_and_key_from_para, Instance, Meta, MetaType, NatureError, Result};
use nature_db::{Mission, RawTask};

use crate::task::{CachedKey, TaskForConvert};

pub struct Converted {
    pub done_task: RawTask,
    pub converted: Vec<Instance>,
}

impl Converted {
    pub fn gen(task: &TaskForConvert, convert_task: &RawTask, instances: Vec<Instance>, last_state: &Option<Instance>) -> Result<Converted> {
        if instances.is_empty() {
            return Ok(converted_none(convert_task));
        }

        let mut instances = instances;

        // init meta and [from]
        let from = FromInstance::from(&task.from);
        let _ = set_source_and_target_meta(&mut instances, &from, &task.target.to)?;

        // check id
        let _ = check_id(&mut instances, &from, &task.target)?;

        // filter from cache
        let mut instances: Vec<Instance> = if task.check_cache() {
            instances.into_iter().filter(|one| !CachedKey::get(&one.get_key())).collect()
        } else {
            instances
        };

        // verify state
        let _ = verify_state(&task, &mut instances, last_state)?;

        if task.target.id_bridge {
            sys_context_id(&mut instances, &task.target, &from);
            sys_context_para(&mut instances, &task.target, &from);
        }

        // for `MetaType::Loop`
        if task.target.to.get_meta_type() == MetaType::Loop {
            instances.push(gen_instance_for_loop(task));
        }

        // assemble it
        let rtn = Converted {
            done_task: convert_task.to_owned(),
            converted: instances,
        };
        Ok(rtn)
    }
}

/// **Notice** need get sys_context from upstream
fn gen_instance_for_loop(task: &TaskForConvert) -> Instance {
    let mut rtn = Instance::default();
    rtn.meta = task.target.to.meta_string();
    rtn.id = task.from.id;
    rtn.para = task.from.para.clone();
    if let Some(v) = task.from.sys_context.get(CONTEXT_LOOP_NEXT) {
        rtn.sys_context.insert(CONTEXT_LOOP_NEXT.to_string(), v.to_string());
    }
    if let Some(v) = task.from.sys_context.get(CONTEXT_LOOP_FINISHED) {
        rtn.sys_context.insert(CONTEXT_LOOP_FINISHED.to_string(), v.to_string());
    }
    rtn
}

fn converted_none(carrier: &RawTask) -> Converted {
    Converted {
        done_task: carrier.to_owned(),
        converted: Vec::new(),
    }
}

fn set_source_and_target_meta(instances: &mut Vec<Instance>, from: &FromInstance, target_meta: &Meta) -> Result<()> {
    match target_meta.get_meta_type() {
        MetaType::Multi => {
            match target_meta.get_setting() {
                Some(s) => s.check_multi_meta(instances)?,
                None => set_all_instances(instances, from, target_meta),
            }
        }
        MetaType::Loop => {
            match target_meta.get_setting() {
                Some(s) => s.check_multi_meta(instances)?,
                None => set_all_instances(instances, from, target_meta),
            }
        }
        _ => set_all_instances(instances, from, target_meta),
    }
    Ok(())
}

fn set_all_instances(instances: &mut Vec<Instance>, from: &FromInstance, target_meta: &Meta) {
    instances.iter_mut().for_each(|n| {
        n.data.meta = target_meta.meta_string();
        n.from = Some(from.clone());
    });
}


fn check_id(ins: &mut Vec<Instance>, from: &FromInstance, target: &Mission) -> Result<()> {
    if ins.is_empty() {
        return Ok(());
    }
    // handle state-instance, the id and para had put to the sys_context before convert
    if target.to.is_state() {
        if let Some(id) = target.sys_context.get(CONTEXT_TARGET_INSTANCE_ID) {
            ins[0].id = match u128::from_str_radix(id, 16) {
                Ok(rtn) => rtn,
                Err(e) => {
                    let msg = format!("sys_context.{}, {} can'nt convert to u128, err:{}", CONTEXT_TARGET_INSTANCE_ID, id, e);
                    return Err(NatureError::VerifyError(msg));
                }
            }
        }
        if let Some(para) = target.sys_context.get(CONTEXT_TARGET_INSTANCE_PARA) {
            ins[0].para = para.to_string();
        }
        if ins[0].id == 0 && ins[0].para.is_empty() {
            ins[0].revise()?;
        }
        return Ok(());
    }

    // handle normal-instance
    let id = {
        if target.use_upstream_id || target.to.check_master(&from.meta) {
            Some(from.id)
        } else { None }
    };

    for mut one in ins {
        if let Some(id_u) = id {
            one.id = id_u;
        } else {
            one.revise()?;
        }
        if target.target_demand.copy_para.len() > 0 {
            let result = get_para_and_key_from_para(&from.para, &target.target_demand.copy_para)?;
            one.para = result.0;
        }
    }
    Ok(())
}

fn sys_context_id(instances: &mut Vec<Instance>, mission: &Mission, from: &FromInstance) {
    if let Some(id) = mission.sys_context.get(CONTEXT_TARGET_INSTANCE_ID) {
        for instance in instances {
            instance.data.sys_context.insert(CONTEXT_TARGET_INSTANCE_ID.to_string(), id.to_string());
        }
    } else {
        for instance in instances {
            instance.data.sys_context.insert(CONTEXT_TARGET_INSTANCE_ID.to_string(), format!("{:x}", from.id));
        }
    }
}

fn sys_context_para(instances: &mut Vec<Instance>, mission: &Mission, from: &FromInstance) {
    if let Some(para) = mission.sys_context.get(CONTEXT_TARGET_INSTANCE_PARA) {
        for instance in instances {
            instance.data.sys_context.insert(CONTEXT_TARGET_INSTANCE_PARA.to_string(), para.to_string());
        }
    } else if !from.para.is_empty() {
        for instance in instances {
            instance.data.sys_context.insert(CONTEXT_TARGET_INSTANCE_PARA.to_string(), from.para.to_string());
        }
    }
}

fn verify_state(task: &TaskForConvert, instances: &mut Vec<Instance>, last_state: &Option<Instance>) -> Result<()> {
    let to = &task.target.to;
    if !to.is_state() {
        return Ok(());
    }
    if instances.len() > 1 {
// only one status instance should return
        return Err(NatureError::LogicalError("[status meta] must return less 2 instances!".to_string()));
    }
    let ins = &mut instances[0];

// states and state version
    let temp_states = ins.states.clone();
    match last_state {
        None => {
            if task.from.meta == task.target.to.meta_string() {
                ins.state_version = task.from.state_version + 1;
            } else {
                ins.state_version = 1;
            }
        }
        Some(x) => {
            ins.state_version = x.state_version + 1;
            ins.states = x.states.clone();
        }
    };
// set status
    if let Some(lsd) = &task.target.target_demand.states {
        ins.modify_state(lsd, &task.target.to);
    } else {
        let (_, mutex) = task.target.to.check_state(&temp_states.clone().into_iter().collect())?;
        if mutex.len() > 0 {
            return Err(NatureError::LogicalError(format!("returned mutex state {:?}", mutex)));
        }
        ins.states = temp_states
    }
    Ok(())
}

#[cfg(test)]
mod sys_context_test {
    use super::*;

    #[test]
    fn id_from_previous_id() {
        let mut ins: Vec<Instance> = vec![Instance::default()];
        let mut mission = Mission::default();
        mission.id_bridge = true;
        let mut from = FromInstance::default();
        from.id = 123;
        sys_context_id(&mut ins, &mission, &from);
        assert_eq!("7b", ins[0].sys_context.get("target.id").unwrap());
    }

    #[test]
    fn id_from_previous_context() {
        let mut ins: Vec<Instance> = vec![Instance::default()];
        let mut mission = Mission::default();
        mission.sys_context.insert("target.id".to_string(), "abc".to_string());
        mission.id_bridge = true;
        sys_context_id(&mut ins, &mission, &FromInstance::default());
        assert_eq!("abc", ins[0].sys_context.get("target.id").unwrap());
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;

    use nature_common::{Meta, MetaType, State, TargetState};
    use nature_db::Mission;
    use nature_db::relation_target::RelationTarget;

    use super::*;

    #[test]
    fn upstream_test() {
        let mut from_ins = Instance::default();
        from_ins.id = 567;
        from_ins.meta = "B:from:1".to_string();
        from_ins.state_version = 2;
        let meta = Meta::new("to", 1, MetaType::Business).unwrap();
        let task_key = from_ins.get_key();
        let task = TaskForConvert {
            from: from_ins,
            target: Mission {
                to: meta.clone(),
                executor: Default::default(),
                filter_before: vec![],
                filter_after: vec![],
                target_demand: Default::default(),
                use_upstream_id: true,
                delay: 0,
                sys_context: Default::default(),
                id_bridge: false,
            },
            conflict_version: 0,
        };
        let raw = RawTask {
            task_id: "".to_string(),
            task_key,
            task_type: 0,
            task_for: "".to_string(),
            data: "".to_string(),
            create_time: Local::now().naive_local(),
            execute_time: Local::now().naive_local(),
            retried_times: 0,
            task_state: 0,
        };
        let mut ins = Instance::default();
        ins.id = 123;
        let ins = vec![ins];

        let result = Converted::gen(&task, &raw, ins.clone(), &None).unwrap();
        let c = &result.converted[0];
        let from = c.from.as_ref().unwrap();
        assert_eq!(from.id, 567);
        assert_eq!(from.meta, "B:from:1");
        assert_eq!(from.state_version, 2);
        assert_eq!(result.converted[0].id, 567);
    }

    #[test]
    fn target_states_test() {
        let task = TaskForConvert {
            from: Default::default(),
            target: Mission {
                to: {
                    let mut m = Meta::from_string("B:hello:1").unwrap();
                    let _ = m.set_states(Some(vec![State::Normal("new".to_string())]));
                    m
                },
                executor: Default::default(),
                filter_before: vec![],
                filter_after: vec![],
                target_demand: RelationTarget {
                    states: Some({
                        let mut sd = TargetState::default();
                        sd.add = Some(vec!["new".to_string()]);
                        sd
                    }),
                    copy_para: vec![],
                },
                use_upstream_id: false,
                delay: 0,
                sys_context: Default::default(),
                id_bridge: false,
            },
            conflict_version: 0,
        };
        let mut ins = vec![Instance::new("test").unwrap()];
        let _ = verify_state(&task, &mut ins, &None);
        let one = &ins[0];
        assert_eq!(one.states.contains("new"), true)
    }
}

#[cfg(test)]
mod check_id_for_state {
    use nature_common::{Meta, MetaSetting, MetaType};

    use super::*;

    #[test]
    fn input_is_empty() {
        let (from, mission) = init_input();
        let rtn = check_id(&mut vec![], &from, &mission);
        assert_eq!(rtn.is_ok(), true)
    }

    #[test]
    fn sys_context_nothing() {
        let (from, mission) = init_input();
        let sta = Instance::default();
        let vec = &mut vec![sta];
        assert_eq!(vec[0].id, 0);
        let rtn = check_id(vec, &from, &mission);
        assert_eq!(rtn.is_ok(), true);
        assert_eq!(vec[0].id > 0, true)
    }

    #[test]
    fn sys_context_for_id() {
        let (from, mut mission) = init_input();
        let sta = Instance::default();
        mission.sys_context.insert("target.id".to_string(), "5".to_string());
        let vec = &mut vec![sta];
        assert_eq!(vec[0].id, 0);
        let rtn = check_id(vec, &from, &mission);
        assert_eq!(rtn.is_ok(), true);
        assert_eq!(vec[0].id, 5)
    }

    #[test]
    fn only_para_for_context() {
        let (from, mut mission) = init_input();
        let sta = Instance::default();
        mission.sys_context.insert("target.para".to_string(), "a".to_string());
        let vec = &mut vec![sta];
        let rtn = check_id(vec, &from, &mission);
        assert_eq!(rtn.is_ok(), true);
        assert_eq!(vec[0].id, 0);
        assert_eq!(vec[0].para, "a")
    }

    /// master is from
    fn init_input() -> (FromInstance, Mission) {
        let from = FromInstance::default();

        let mut setting = MetaSetting::default();
        setting.is_state = true;

        let mut meta = Meta::new("to", 1, MetaType::Business).unwrap();
        let _ = meta.set_setting(&setting.to_json().unwrap());

        let mut mission = Mission::default();
        mission.to = meta;

        (from, mission)
    }
}

#[cfg(test)]
mod check_id_for_normal {
    use nature_common::{Meta, MetaSetting, MetaType};

    use super::*;

    #[test]
    fn normal() {
        let (from, mut mission) = init_input();
        mission.to = Meta::new("remove master", 1, MetaType::Business).unwrap();
        let one = Instance::new("one").unwrap();
        let mut input = vec![one];
        let _ = check_id(&mut input, &from, &mission);
        assert_eq!(input[0].id != 123, true);
        assert_eq!(input[0].id != 0, true);
    }

    #[test]
    fn use_upstream_id() {
        let (from, mut mission) = init_input();
        mission.to = Meta::new("remove master", 1, MetaType::Business).unwrap();
        mission.use_upstream_id = true;
        let one = Instance::new("one").unwrap();
        let mut input = vec![one];
        let _ = check_id(&mut input, &from, &mission);
        assert_eq!(input[0].id, 123);
    }


    /// the instance that will be saved is child of from
    #[test]
    fn use_master_id() {
        let (from, mission) = init_input();
        let one = Instance::new("one").unwrap();
        let mut input = vec![one];
        let _ = check_id(&mut input, &from, &mission);
        assert_eq!(input[0].id, 123);
    }

    /// master is from
    fn init_input() -> (FromInstance, Mission) {
        let from = FromInstance {
            id: 123,
            meta: "from".to_string(),
            para: "".to_string(),
            state_version: 1,
        };

        let mut setting = MetaSetting::default();
        setting.master = Some("from".to_string());

        let mut meta = Meta::new("to", 1, MetaType::Business).unwrap();
        let _sr = meta.set_setting(&setting.to_json().unwrap());

        let mut mission = Mission::default();
        mission.to = meta;
        (from, mission)
    }
}