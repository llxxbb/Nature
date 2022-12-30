use std::convert::TryInto;
use std::ops::Add;

use chrono::{FixedOffset, Local};
use futures::Future;
use crate::common::*;

use crate::db::{MetaCache, MetaDao, Mission, MissionRaw, RawTask, TaskType};
use crate::domain::*;
use crate::nature_lib::task::TaskForStore;

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
                car.execute_time = Local::now().add(FixedOffset::east_opt(c.delay).unwrap()).naive_local()
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
    pub async fn from_raw<T, MC, M>(raw: &RawTask, ins_g: fn(InsCond) -> T, mc_g: &MC, m_g: &M) -> Result<Self>
        where T: Future<Output=Result<Option<Instance>>>, MC: MetaCache, M: MetaDao
    {
        let mr = MissionRaw::from_json(&raw.data)?;
        let result = ins_g(raw.try_into()?).await?;
        let rtn = match result {
            None => return Err(NatureError::EnvironmentError("can't find instance".to_string())),
            Some(ins) => {
                TaskForConvert {
                    from: ins,
                    target: Mission::from_raw(&mr, mc_g, m_g).await?,
                    conflict_version: 0,
                }
            }
        };
        Ok(rtn)
    }
}
