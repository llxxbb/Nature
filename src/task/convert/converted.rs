use nature_common::{Instance, MetaType, NatureError, Result};
use nature_db::{MetaCacheGetter, MetaGetter, RawMeta, RawTask};

use crate::task::TaskForConvert;

pub struct Converted {
    pub done_task: RawTask,
    pub converted: Vec<Instance>,
}

impl Converted {
    pub fn gen(task: &TaskForConvert, carrier: &RawTask, instances: Vec<Instance>, meta_cache_getter: MetaCacheGetter, meta_getter: MetaGetter) -> Result<Converted> {
        // check `MetaType` for Null
        if task.target.to.get_meta_type() == MetaType::Null {
            let rtn = Converted {
                done_task: carrier.to_owned(),
                converted: Vec::new(),
            };
            return Ok(rtn);
        }

        let mut fixxed_ins: Vec<Instance> = Vec::new();
        // fix id and modify the meta
        for one in instances {
            let mut n = one.clone();
            n.data.meta = task.target.to.clone();
            let _ = n.fix_id();
            fixxed_ins.push(n)
        }
        // verify
        let instances = Self::verify(&task, &fixxed_ins, meta_cache_getter, meta_getter)?;
        let rtn = Converted {
            done_task: carrier.to_owned(),
            converted: instances,
        };
        Ok(rtn)
    }

    fn verify(task: &TaskForConvert, instances: &[Instance], meta_cache_getter: MetaCacheGetter, meta_getter: MetaGetter) -> Result<Vec<Instance>> {
        let mut rtn: Vec<Instance> = Vec::new();
        // only one status instance should return
        let to = task.target.to.clone();
        let define = match to.get_meta_type() {
            MetaType::Dynamic => RawMeta::default(),
            _ => meta_cache_getter(&to, meta_getter)?
        };
        if task.target.use_upstream_id && instances.len() > 1 {
            return Err(NatureError::ConverterLogicalError("[use_upstream_id] must return less 2 instances!".to_string()));
        }
        if define.has_states() {
            if instances.len() > 1 {
                return Err(NatureError::ConverterLogicalError("[status meta] must return less 2 instances!".to_string()));
            }
            // status version must equal old + 1
            if instances.len() == 1 {
                let mut ins = instances[0].clone();
                if task.target.use_upstream_id {
                    ins.id = task.from.id;
                }
                match &task.last_status {
                    None => {
                        ins.state_version = 1;
                    }
                    Some(x) => {
                        ins.state_version = x.state_version + 1;
                        ins.states = x.states.clone();
                    }
                };
                if let Some(lsd) = &task.target.last_states_demand {
                    if let Some(ts) = &lsd.target_states {
                        ins.modify_state(ts.clone());
                    }
                }
                ins.data.meta = to.clone();
                rtn.push(ins);
            }
            return Ok(rtn);
        }

        // all biz must same to "to" and set id
        for r in instances {
            let mut instance = r.clone();
            if task.target.use_upstream_id {
                instance.id = task.from.id;
            }
            instance.data.meta = to.clone();
            let _ = instance.fix_id();
            rtn.push(instance);
        }

        Ok(rtn)
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;

    use nature_common::{Meta, State};
    use nature_db::{MetaDaoImpl, Mission};

    use super::*;

    #[test]
    fn use_upstream_id() {
        let mut from_ins = Instance::default();
        from_ins.id = 567;
        let meta = Meta::new("to").unwrap();
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
        let result = Converted::gen(&task, &raw, ins.clone(), mate_to_raw, MetaDaoImpl::get).unwrap();
        assert_eq!(result.converted[0].id, 567);

        // for state
        task.target.to.state = Some(vec![State::Normal("hello".to_string())]);
        let result = Converted::gen(&task, &raw, ins, mate_to_raw, MetaDaoImpl::get).unwrap();
        assert_eq!(result.converted[0].id, 567);
    }

    fn mate_to_raw(_: &Meta, _: MetaGetter) -> Result<RawMeta> {
        Ok(RawMeta {
            full_key: "".to_string(),
            description: None,
            version: 0,
            states: Some("a,b,c".to_string()),
            fields: None,
            config: "".to_string(),
            flag: 0,
            create_time: Local::now().naive_local(),
        })
    }
}