use std::ops::Add;

use chrono::{FixedOffset, Local};

use nature_common::{Instance, Result};
use nature_db::{Mission, RawTask, TaskType};

use crate::task::TaskForStore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskForConvert {
    pub from: Instance,
    pub target: Mission,
}

impl Default for TaskForConvert {
    fn default() -> Self {
        TaskForConvert {
            from: Instance::default(),
            target: Mission::default(),
        }
    }
}

impl TaskForConvert {
    pub fn gen_task(task: &TaskForStore) -> Result<Vec<(TaskForConvert, RawTask)>>
    {
        let mut new_carriers: Vec<(TaskForConvert, RawTask)> = Vec::new();
        let missions = task.next_mission.clone().unwrap();
        for c in missions {
            let x = TaskForConvert {
                from: task.instance.clone(),
                target: c.clone(),
            };
            debug!("--generate convert task: from:{}, to:{},", x.from.meta, x.target.to.meta_string());
            let mut car = RawTask::new(&x, &c.to.meta_string(), TaskType::Convert as i16)?;
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
}

#[cfg(test)]
mod test {
    use std::ops::Add;

    use chrono::{FixedOffset, Local};

    #[allow(dead_code)]
    fn time_add_test() {
        let a = Local::now();
        let b = a.add(FixedOffset::east(1));
        let x = a.signed_duration_since(b);
        dbg!(x);
    }
}
