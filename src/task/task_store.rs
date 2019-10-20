use std::sync::mpsc::Sender;

use nature_common::{DynamicConverter, Instance, Result};
use nature_db::{Mission, RawTask};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskForStore {
    pub instance: Instance,
    pub next_mission: Option<Vec<Mission>>,
    pub previous_mission: Option<Mission>,
}

impl TaskForStore {
    pub fn send(&self, raw: &RawTask, sender: &Sender<(TaskForStore, RawTask)>) {
        let _ = sender.send((self.to_owned(), raw.to_owned()));
    }

    pub fn for_dynamic(instance: &Instance, dynamic: Vec<DynamicConverter>) -> Result<TaskForStore> {
        let target = Mission::for_dynamic(dynamic)?;
        // save to task to make it can redo
        let task = TaskForStore::new(instance.clone(), Some(target));
        Ok(task)
    }

    pub fn new(instance: Instance, next_mission: Option<Vec<Mission>>) -> Self {
        TaskForStore {
            instance,
            next_mission,
            previous_mission: None,
        }
    }

    pub fn new_with_previous_mission(instance: Instance, next_mission: Option<Vec<Mission>>, previous_mision: &Mission) -> Self {
        let previous_mission = match previous_mision.to.is_state() {
            true => Some(previous_mision.clone()),
            false => None
        };
        TaskForStore {
            instance,
            next_mission,
            previous_mission,
        }
    }
}