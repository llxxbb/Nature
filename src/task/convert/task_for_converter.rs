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
            let car = RawTask::new(&x, &c.to.meta_string(), TaskType::Convert as i16)?;
            new_carriers.push((x, car));
        }
        Ok(new_carriers)
    }
}
