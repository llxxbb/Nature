use nature_common::{Instance, MetaType, NatureError, Result};
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

        // fix id and modify the meta
        let mut instances = instances;
        instances.iter_mut().for_each(|n| {
            n.data.meta = task.target.to.get_string();
            if task.target.use_upstream_id {
                n.id = task.from.id;
            }
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
        match &task.last_status {
            None => {
                ins.state_version = 1;
            }
            Some(x) => {
                ins.state_version = x.state_version + 1;
                ins.states = x.states.clone();
            }
        };
        // target
        if let Some(lsd) = &task.target.last_states_demand {
            if let Some(ts) = &lsd.target_states {
                ins.modify_state(ts.clone());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;

    use nature_common::{Meta, State};
    use nature_db::Mission;

    use super::*;

    #[test]
    fn use_upstream_id() {
        let mut from_ins = Instance::default();
        from_ins.id = 567;
        let meta = Meta::new("to", 1, MetaType::Business).unwrap();
        let mut task = TaskForConvert {
            from: from_ins,
            target: Mission {
                to: meta.clone(),
                executor: Default::default(),
                last_states_demand: None,
                use_upstream_id: true,
            },
            last_status: None,
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
        assert_eq!(result.converted[0].id, 567);

        // for state
        task.target.to.state = Some(vec![State::Normal("hello".to_string())]);
        let result = Converted::gen(&task, &raw, ins).unwrap();
        assert_eq!(result.converted[0].id, 567);
    }
}