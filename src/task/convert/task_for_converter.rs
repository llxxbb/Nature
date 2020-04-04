use std::ops::Add;

use chrono::{FixedOffset, Local};

use nature_common::{FromInstance, Instance, NatureError, ParaForQueryByID, Result};
use nature_db::{InstanceGetter, MetaCacheGetter, MetaGetter, Mission, MissionRaw, RawTask, TaskType};

use crate::task::TaskForStore;

#[derive(Debug, Clone)]
pub struct TaskForConvert {
    pub from: Instance,
    pub target: Mission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskForConvertTemp {
    pub from: FromInstance,
    pub target: MissionRaw,
}

impl Default for TaskForConvert {
    fn default() -> Self {
        TaskForConvert {
            from: Instance::default(),
            target: Mission::default(),
        }
    }
}

impl From<TaskForConvert> for TaskForConvertTemp {
    fn from(input: TaskForConvert) -> Self {
        TaskForConvertTemp {
            from: FromInstance::from(&input.from),
            target: MissionRaw::from(input.target),
        }
    }
}

impl TaskForConvert {
    pub fn gen_task(task: &TaskForStore) -> Result<Vec<(TaskForConvert, RawTask)>>
    {
        let mut new_carriers: Vec<(TaskForConvert, RawTask)> = Vec::new();
        let missions = task.next_mission.clone();
        for c in missions {
            let from = task.instance.clone();
            let key = from.get_key();
            let x = TaskForConvert {
                from,
                target: c.clone(),
            };
            // debug!("generate convert task: from:{}, to:{},", x.from.meta, x.target.to.meta_string());
            let temp = TaskForConvertTemp::from(x.clone());
            let json = serde_json::to_string(&temp)?;
            let mut car = RawTask::from_str(&json, &key, TaskType::Convert as i8, &c.to.meta_string())?;
            if c.delay > 0 {
                car.execute_time = Local::now().add(FixedOffset::east(c.delay)).naive_local()
            }
            new_carriers.push((x, car));
        }
        Ok(new_carriers)
    }
    pub fn check_cache(&self) -> bool {
        match self.target.to.get_setting() {
            Some(s) => s.conflict_avoid,
            None => false,
        }
    }
    pub fn from_raw(json: &str, ins_g: InstanceGetter, mc_g: MetaCacheGetter, m_g: &MetaGetter) -> Result<Self> {
        let temp = serde_json::from_str::<TaskForConvertTemp>(json)?;
        let q_para = ParaForQueryByID::from(&temp.from);
        let result = ins_g(&q_para)?;
        let rtn = match result {
            None => return Err(NatureError::EnvironmentError("can't find instance".to_string())),
            Some(ins) => {
                TaskForConvert {
                    from: ins,
                    target: Mission::from_raw(&temp.target, mc_g, m_g)?,
                }
            }
        };
        Ok(rtn)
    }
}
