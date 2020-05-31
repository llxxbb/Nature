use std::ops::Add;

use chrono::{FixedOffset, Local};
use futures::Future;

use nature_common::{Instance, NatureError, Result};
use nature_db::{MetaCacheGetter, MetaGetter, Mission, MissionRaw, RawTask, TaskType};

use crate::task::{TASK_KEY_SEPARATOR, TaskForStore};

#[derive(Debug, Clone)]
pub struct TaskForConvert {
    pub from: Instance,
    pub target: Mission,
    pub conflict_version: i32,
}

impl Default for TaskForConvert {
    fn default() -> Self {
        TaskForConvert {
            from: Instance::default(),
            target: Mission::default(),
            conflict_version: 0,
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
                conflict_version: 0,
            };
            // debug!("generate convert task: from:{}, to:{},", x.from.meta, x.target.to.meta_string());
            let json = MissionRaw::from(x.target.clone()).to_json()?;
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
            Some(s) => {
                s.cache_saved
            }
            None => false,
        }
    }
    pub async fn from_raw<T>(raw: &RawTask, ins_g: fn(String, String) -> T, mc_g: MetaCacheGetter, m_g: &MetaGetter) -> Result<Self>
        where T: Future<Output=Result<Option<Instance>>>
    {
        let mr = MissionRaw::from_json(&raw.data)?;
        let result = ins_g(raw.task_key.to_string(), TASK_KEY_SEPARATOR.to_string()).await?;
        let rtn = match result {
            None => return Err(NatureError::EnvironmentError("can't find instance".to_string())),
            Some(ins) => {
                TaskForConvert {
                    from: ins,
                    target: Mission::from_raw(&mr, mc_g, m_g)?,
                    conflict_version: 0,
                }
            }
        };
        Ok(rtn)
    }
}
