use nature_common::{FromInstance, Instance, MetaType, NatureError, Result};
use nature_db::RawTask;

use crate::task::TaskForConvert;

pub struct Converted {
    pub done_task: RawTask,
    pub converted: Vec<Instance>,
}

impl Converted {
    pub fn gen(task: &TaskForConvert, carrier: &RawTask, instances: Vec<Instance>) -> Result<Converted> {
        // check `MetaType` for Null
        if task.target.to.get_meta_type() == MetaType::Null {
            let rtn = Converted {
                done_task: carrier.to_owned(),
                converted: Vec::new(),
            };
            return Ok(rtn);
        }

        if instances.is_empty() {
            let msg = format!("Must return instance for meta: {}", task.target.to.get_string());
            warn!("{}", &msg);
            return Err(NatureError::VerifyError(msg));
        }

        // fix id, meta and From
        let from = FromInstance {
            id: task.from.id,
            meta: task.from.meta.to_string(),
            state_version: task.from.state_version,
        };
        let mut instances = instances;
        instances.iter_mut().for_each(|n| {
            n.data.meta = task.target.to.get_string();
            if task.target.use_upstream_id {
                n.id = task.from.id;
            }
            n.from = Some(from.clone());
            let _ = n.revise();
        });

        // verify
        let _ = Self::verify_state(&task, &mut instances)?;
        let rtn = Converted {
            done_task: carrier.to_owned(),
            converted: instances,
        };
        Ok(rtn)
    }

    fn verify_state(task: &TaskForConvert, instances: &mut Vec<Instance>) -> Result<()> {
        let to = &task.target.to;
        if !to.is_state {
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
        match &task.last_state {
            None => {
                ins.state_version = 1;
            }
            Some(x) => {
                ins.state_version = x.state_version + 1;
                ins.states = x.states.clone();
            }
        };
        // set status
        if let Some(lsd) = &task.target.states_demand {
            if let Some(ts) = &lsd.target_states {
                ins.modify_state(ts);
            }
        }else{
            ins.modify_state()
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
            last_state: None,
        };
        let raw = RawTask {
            task_id: vec![],
            meta: "".to_string(),
            data_type: 0,
            data: "".to_string(),
            create_time: Local::now().naive_local(),
            execute_time: Local::now().naive_local(),
            retried_times: 0,
        };
        let mut ins = Instance::default();
        ins.id = 123;
        let ins = vec![ins];

        // for normal
        let result = Converted::gen(&task, &raw, ins.clone()).unwrap();
        let c = &result.converted[0];
        let from = c.from.as_ref().unwrap();
        assert_eq!(from.id, 567);
        assert_eq!(from.meta, "/B/from:1");
        assert_eq!(from.state_version, 2);
        assert_eq!(result.converted[0].id, 567);

        // for state
        task.target.to.state = Some(vec![State::Normal("hello".to_string())]);
        let result = Converted::gen(&task, &raw, ins).unwrap();
        assert_eq!(result.converted[0].id, 567);
    }

    #[test]
    fn target_states_test() {
        let task = TaskForConvert {
            from: Default::default(),
            target: Mission {
                to: {
                    let mut m = Meta::from_string("/B/hello:1").unwrap();
                    m.is_state = true;
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
            last_state: None,
        };
        let mut ins = vec![Instance::new("test").unwrap()];
        let _ = Converted::verify_state(&task, &mut ins);
        let one = &ins[0];
        assert_eq!(one.states.contains("new"), true)
    }
}