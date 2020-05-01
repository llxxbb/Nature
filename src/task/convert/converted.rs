use std::str::FromStr;

use nature_common::{CONTEXT_TARGET_INSTANCE_ID, FromInstance, Instance, Meta, MetaType, NatureError, Result};
use nature_db::{Mission, RawTask};

use crate::task::{CachedKey, TaskForConvert};

pub struct Converted {
    pub done_task: RawTask,
    pub converted: Vec<Instance>,
}

impl Converted {
    pub fn gen(task: &TaskForConvert, convert_task: &RawTask, instances: Vec<Instance>, last_state: &Option<Instance>) -> Result<Converted> {

        // filter from cache
        let mut instances: Vec<Instance> = if task.check_cache() {
            instances.into_iter().filter(|one| !CachedKey::get(&one.get_unique())).collect()
        } else {
            instances
        };

        if instances.is_empty() {
            return Ok(converted_none(convert_task));
        }

        // init meta and [from]
        let from = FromInstance::from(&task.from);
        let _ = set_source_and_target_meta(&mut instances, &from, &task.target.to)?;

        // check id
        let _ = check_id(&mut instances, &last_state, &from, &task.target)?;

        // verify
        let _ = verify_state(&task, &mut instances, last_state)?;
        let rtn = Converted {
            done_task: convert_task.to_owned(),
            converted: instances,
        };
        Ok(rtn)
    }
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
        _ => set_all_instances(instances, from, target_meta),
    }
    Ok(())
}

fn set_all_instances(instances: &mut Vec<Instance>, from: &FromInstance, target_meta: &Meta) {
    instances.iter_mut().for_each(|n| {
        n.data.meta = target_meta.meta_string();
        n.from = Some(from.clone());
        let _ = n.revise();
    });
}


fn check_id(ins: &mut Vec<Instance>, last: &Option<Instance>, from: &FromInstance, target: &Mission) -> Result<()> {
    if ins.is_empty() {
        return Ok(());
    }

    let is_master = match &target.to.get_setting() {
        Some(setting) => match &setting.master {
            None => false,
            Some(master) => master.eq(&from.meta)
        }
        _ => false
    };

    let id = {
        if target.use_upstream_id || is_master {
            Some(from.id)
        } else if target.to.is_state() && last.is_some() {
            Some(last.as_ref().unwrap().id)
        } else { None }
    };

    for mut one in ins {
        if let Some(id_u) = id {
            one.id = id_u;
        } else if let Some(id_s) = one.sys_context.get(&*CONTEXT_TARGET_INSTANCE_ID) {
            one.id = u128::from_str(id_s)?;
        }
    }
    Ok(())
}

fn verify_state(task: &TaskForConvert, instances: &mut Vec<Instance>, last_state: &Option<Instance>) -> Result<()> {
    let to = &task.target.to;
    if !to.is_state() {
        return Ok(());
    }
    if task.target.use_upstream_id && instances.len() > 1 {
        return Err(NatureError::LogicalError("[use_upstream_id] must return less 2 instances!".to_string()));
    }
    if instances.len() > 1 {
// only one status instance should return
        return Err(NatureError::LogicalError("[status meta] must return less 2 instances!".to_string()));
    }
    let mut ins = &mut instances[0];
// upstream id
    if task.target.use_upstream_id {
        ins.id = task.from.id;
    }
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
            ins.id = x.id;
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
        let mut task = TaskForConvert {
            from: from_ins,
            target: Mission {
                to: meta.clone(),
                executor: Default::default(),
                filter_before: vec![],
                filter_after: vec![],
                target_demand: Default::default(),
                use_upstream_id: true,
                delay: 0,
            },
            conflict_version: 0,
        };
        let raw = RawTask {
            task_id: vec![],
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

// for normal
        let result = Converted::gen(&task, &raw, ins.clone(), &None).unwrap();
        let c = &result.converted[0];
        let from = c.from.as_ref().unwrap();
        assert_eq!(from.id, 567);
        assert_eq!(from.meta, "B:from:1");
        assert_eq!(from.state_version, 2);
        assert_eq!(result.converted[0].id, 567);

// for state
        let _ = task.target.to.set_states(Some(vec![State::Normal("hello".to_string())]));
        let result = Converted::gen(&task, &raw, ins, &None).unwrap();
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
                    upstream_para: vec![],
                },
                use_upstream_id: false,
                delay: 0,
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
mod check_id_test {
    use nature_common::{Meta, MetaSetting, MetaType};

    use super::*;

    #[test]
    fn vec_is_empty() {
        let (last, from, mission) = init_input();
        let rtn = check_id(&mut vec![], &Some(last), &from, &mission);
        assert_eq!(rtn.is_ok(), true)
    }

    #[test]
    fn vec_more_then_one() {
        let (last, from, mission) = init_input();
        let one = Instance::new("one").unwrap();
        let two = Instance::new("two").unwrap();
        let mut input = vec![one.clone(), two.clone()];
        let _ = check_id(&mut input, &Some(last), &from, &mission);
        assert_eq!(input[0].id, 123);
        assert_eq!(input[1].id, 123);
    }

    #[test]
    fn no_effect_for_none_state_taget() {
        let (last, from, mut mission) = init_input();
        mission.to = Meta::new("noState", 1, MetaType::Business).unwrap();
        let one = Instance::new("one").unwrap();
        let mut input = vec![one.clone()];
        let _ = check_id(&mut input, &Some(last), &from, &mission);
        assert_eq!(input[0].id, 0);
    }

    #[test]
    fn use_last_id() {
        let mut last = Instance::new("last").unwrap();
        last.id = 456;
        let from = FromInstance {
            id: 123,
            meta: "from".to_string(),
            para: "".to_string(),
            state_version: 1,
        };
        let mut meta = Meta::new("to", 1, MetaType::Business).unwrap();
        let setting = MetaSetting {
            is_state: true,
            master: Some("another".to_string()),
            multi_meta: Default::default(),
            cache_saved: false,
        };
        let _ = meta.set_setting(&setting.to_json().unwrap());
        let mission = Mission {
            to: meta,
            executor: Default::default(),
            filter_before: vec![],
            filter_after: vec![],
            target_demand: Default::default(),
            use_upstream_id: false,
            delay: 0,
        };
        let one = Instance::new("one").unwrap();
        let mut input = vec![one.clone()];
        assert_eq!(input[0].id, 0);
        let _ = check_id(&mut input, &Some(last), &from, &mission);
        assert_eq!(input[0].id, 456);
    }

    #[test]
    fn master_not_matched() {
        let (_last, mut from, mission) = init_input();
        from.meta = "not_matched".to_string();
        let one = Instance::new("one").unwrap();
        let mut input = vec![one.clone()];
        let _ = check_id(&mut input, &None, &from, &mission);
        assert_eq!(input[0].id, 0);
    }

    #[test]
    fn master_matched() {
        let (_last, from, mission) = init_input();
        let one = Instance::new("one").unwrap();
        let mut input = vec![one];
        let _ = check_id(&mut input, &None, &from, &mission);
        assert_eq!(input[0].id, 123);
    }

    #[test]
    fn use_upstream_id() {
        let (_last, from, mut mission) = init_input();
        mission.to = Meta::default();
        mission.use_upstream_id = true;
        let one = Instance::new("one").unwrap();
        let mut input = vec![one.clone()];
        let _ = check_id(&mut input, &None, &from, &mission);
        assert_eq!(input[0].id, 123);
    }

    fn init_input() -> (Instance, FromInstance, Mission) {
        let mut last = Instance::new("last").unwrap();
        last.id = 456;
        let from = FromInstance {
            id: 123,
            meta: "from".to_string(),
            para: "".to_string(),
            state_version: 1,
        };
        let mut meta = Meta::new("to", 1, MetaType::Business).unwrap();
        let setting = MetaSetting {
            is_state: true,
            master: Some("from".to_string()),
            multi_meta: Default::default(),
            cache_saved: false,
        };
        let ss = setting.to_json().unwrap();
        let _sr = meta.set_setting(&ss);
        let mission = Mission {
            to: meta,
            executor: Default::default(),
            filter_before: vec![],
            filter_after: vec![],
            target_demand: Default::default(),
            use_upstream_id: false,
            delay: 0,
        };
        (last, from, mission)
    }
}