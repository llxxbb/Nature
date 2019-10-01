use std::sync::mpsc::Sender;

use nature_common::{DynamicConverter, Instance, Result};
use nature_db::{Mission, MissionFilter, RawTask, Relation};

use crate::task::TaskForConvert;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskForStore {
    pub instance: Instance,
    /// save outside has non converter info.
    pub upstream: Option<TaskForConvert>,
    pub mission: Option<Vec<Mission>>,
}

impl TaskForStore {
    pub fn gen_task(instance: &Instance, mission: &Option<Vec<Relation>>, mission_filter: MissionFilter) -> Result<Self> {
        let steps = match mission {
            Some(steps) => {
                mission_filter(&instance, steps)
            }
            None => None
        };
        Ok(
            TaskForStore {
                instance: instance.clone(),
                upstream: None,
                mission: steps,
            }
        )
    }

    pub fn send(&self, raw: &RawTask, sender: &Sender<(TaskForStore, RawTask)>) {
        let _ = sender.send((self.to_owned(), raw.to_owned()));
    }

    pub fn for_dynamic(instance: &Instance, dynamic: Vec<DynamicConverter>) -> Result<TaskForStore> {
        let target = Mission::for_dynamic(dynamic)?;
        // save to task to make it can redo
        let task = TaskForStore {
            instance: instance.clone(),
            upstream: None,
            mission: Some(target),
        };
        Ok(task)
    }
}