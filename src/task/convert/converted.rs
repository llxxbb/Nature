use nature_common::{FromInstance, Instance, MetaType, NatureError, Result};
use nature_db::{Mission, RawTask};

use crate::task::TaskForConvert;

pub struct Converted {
    pub done_task: RawTask,
    pub converted: Vec<Instance>,
}

impl Converted {
    pub fn gen(task: &TaskForConvert, carrier: &RawTask, instances: Vec<Instance>, last_state: &Option<Instance>) -> Result<Converted> {
        // check `MetaType` for Null
        if task.target.to.get_meta_type() == MetaType::Null {
            let rtn = Converted {
                done_task: carrier.to_owned(),
                converted: Vec::new(),
            };
            return Ok(rtn);
        }

        if instances.is_empty() {
            let msg = format!("Must return instance for meta: {}", task.target.to.meta_string());
            warn!("{}", &msg);
            return Err(NatureError::VerifyError(msg));
        }

        let from = FromInstance::from(&task.from);

        // init meta and [from]
        let mut instances = instances;
        instances.iter_mut().for_each(|n| {
            n.data.meta = task.target.to.meta_string();
            n.from = Some(from.clone());
            let _ = n.revise();
        });

        // check id
        Self::check_id(&mut instances, &last_state, &from, &task.target);

        // verify
        let _ = Self::verify_state(&task, &mut instances, last_state)?;
        let rtn = Converted {
            done_task: carrier.to_owned(),
            converted: instances,
        };
        Ok(rtn)
    }

    fn check_id(ins: &mut Vec<Instance>, last: &Option<Instance>, from: &FromInstance, target: &Mission) {
        let id = {
            if ins.len() != 1 {
                None
            } else {
                match target.to.is_state() {
                    true => match last {
                        Some(old) => Some(old.id),
                        None => match &target.to.get_setting() {
                            Some(setting) => match &setting.master {
                                None => None,
                                Some(master) => if master.eq(&from.meta) { Some(from.id) } else { None },
                            }
                            None => None
                        }
                    }
                    false => match target.use_upstream_id {
                        true => Some(from.id),
                        false => None
                    }
                }
            }
        };
        if let Some(id) = id {
            let mut first = ins[0].clone();
            first.id = id;
            ins[0] = first;
        }
    }

    fn verify_state(task: &TaskForConvert, instances: &mut Vec<Instance>, last_state: &Option<Instance>) -> Result<()> {
        let to = &task.target.to;
        if !to.is_state() {
            return Ok(());
        }
        if task.target.use_upstream_id && instances.len() > 1 {
            return Err(NatureError::ConverterLogicalError("[use_upstream_id] must return less 2 instances!".to_string()));
        }
        if instances.len() > 1 {
            // only one status instance should return
            return Err(NatureError::ConverterLogicalError("[status meta] must return less 2 instances!".to_string()));
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
        if let Some(lsd) = &task.target.states_demand {
            if let Some(ts) = &lsd.target_states {
                ins.modify_state(ts, &task.target.to);
            }
        } else {
            let (_, mutex) = task.target.to.check_state(&temp_states.clone().into_iter().collect())?;
            if mutex.len() > 0 {
                return Err(NatureError::ConverterLogicalError(format!("returned mutex state {:?}", mutex)));
            }
            ins.states = temp_states
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;

    use nature_common::{Meta, State, TargetState};
    use nature_db::{Mission, StateDemand};

    use super::*;

    #[test]
    fn upstream_test() {
        let mut from_ins = Instance::default();
        from_ins.id = 567;
        from_ins.meta = "/B/from:1".to_string();
        from_ins.state_version = 2;
        let meta = Meta::new("to", 1, MetaType::Business).unwrap();
        let mut task = TaskForConvert {
            from: from_ins,
            target: Mission {
                to: meta.clone(),
                executor: Default::default(),
                states_demand: None,
                use_upstream_id: true,
            },
        };
        let raw = RawTask {
            task_id: vec![],
            meta: "".to_string(),
            data_type: 0,
            data: "".to_string(),
            last_state_version: 0,
            create_time: Local::now().naive_local(),
            execute_time: Local::now().naive_local(),
            retried_times: 0,
        };
        let mut ins = Instance::default();
        ins.id = 123;
        let ins = vec![ins];

        // for normal
        let result = Converted::gen(&task, &raw, ins.clone(), &None).unwrap();
        let c = &result.converted[0];
        let from = c.from.as_ref().unwrap();
        assert_eq!(from.id, 567);
        assert_eq!(from.meta, "/B/from:1");
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
                    let mut m = Meta::from_string("/B/hello:1").unwrap();
                    let _ = m.set_states(Some(vec![State::Normal("new".to_string())]));
                    m
                },
                executor: Default::default(),
                states_demand: Some(StateDemand {
                    last_states_include: Default::default(),
                    last_states_exclude: Default::default(),
                    target_states: Some(TargetState {
                        add: Some(vec!["new".to_string()]),
                        remove: None,
                    }),
                }),
                use_upstream_id: false,
            },
        };
        let mut ins = vec![Instance::new("test").unwrap()];
        let _ = Converted::verify_state(&task, &mut ins, &None);
        let one = &ins[0];
        assert_eq!(one.states.contains("new"), true)
    }
}

#[cfg(test)]
mod check_id_test {
    use nature_common::{Meta, MetaSetting};

    use super::*;

    #[test]
    fn vec_is_empty() {
        let (last, from, mission) = init_input();
        Converted::check_id(&mut vec![], &Some(last), &from, &mission)
    }

    #[test]
    fn vec_more_then_one() {
        let (last, from, mission) = init_input();
        let one = Instance::new("one").unwrap();
        let two = Instance::new("two").unwrap();
        let mut input = vec![one.clone(), two.clone()];
        Converted::check_id(&mut input, &Some(last), &from, &mission);
        assert_eq!(one, input[0]);
        assert_eq!(two, input[1]);
    }

    #[test]
    fn no_effect_for_none_state_taget() {
        let (last, from, mut mission) = init_input();
        mission.to = Meta::new("noState", 1, MetaType::Business).unwrap();
        let one = Instance::new("one").unwrap();
        let mut input = vec![one.clone()];
        Converted::check_id(&mut input, &Some(last), &from, &mission);
        assert_eq!(input[0].id, 0);
    }

    #[test]
    fn use_last_id() {
        let (last, from, mission) = init_input();
        let one = Instance::new("one").unwrap();
        let mut input = vec![one.clone()];
        assert_eq!(input[0].id, 0);
        Converted::check_id(&mut input, &Some(last), &from, &mission);
        assert_eq!(input[0].id, 456);
    }

    #[test]
    fn master_not_matched() {
        let (_last, mut from, mission) = init_input();
        from.meta = "not_matched".to_string();
        let one = Instance::new("one").unwrap();
        let mut input = vec![one.clone()];
        Converted::check_id(&mut input, &None, &from, &mission);
        assert_eq!(input[0].id, 0);
    }

    #[test]
    fn master_matched() {
        let (_last, from, mission) = init_input();
        let one = Instance::new("one").unwrap();
        let mut input = vec![one.clone()];
        Converted::check_id(&mut input, &None, &from, &mission);
        assert_eq!(input[0].id, 123);
    }

    #[test]
    fn use_upstream_id() {
        let (_last, from, mut mission) = init_input();
        mission.to = Meta::default();
        mission.use_upstream_id = true;
        let one = Instance::new("one").unwrap();
        let mut input = vec![one.clone()];
        Converted::check_id(&mut input, &None, &from, &mission);
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
        };
        let _ = meta.set_setting(&serde_json::to_string(&setting).unwrap());
        let mission = Mission {
            to: meta,
            executor: Default::default(),
            states_demand: None,
            use_upstream_id: false,
        };
        (last, from, mission)
    }
}