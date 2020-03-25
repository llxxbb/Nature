use std::sync::mpsc::Sender;

use nature_common::{DynamicConverter, Instance, Result};
use nature_db::{Mission, RawTask};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskForStore {
    pub instance: Instance,
    pub next_mission: Option<Vec<Mission>>,
    /// when state-instance saved conflict, use this property to redo the task
    pub previous_mission: Option<Mission>,
    /// to avoid save conflict
    pub need_cache: bool,
}

impl TaskForStore {
    pub fn send(&self, raw: &RawTask, sender: &Sender<(TaskForStore, RawTask)>) {
        let _ = sender.send((self.to_owned(), raw.to_owned()));
    }

    pub fn for_dynamic(instance: &Instance, dynamic: Vec<DynamicConverter>, previous_mission: Option<Mission>, need_cache: bool) -> Result<TaskForStore> {
        let target = Mission::for_dynamic(dynamic)?;
        // save to task to make it can redo
        let task = TaskForStore::new(instance.clone(), Some(target), previous_mission, need_cache);
        Ok(task)
    }

    pub fn new(instance: Instance, next_mission: Option<Vec<Mission>>, previous_mission: Option<Mission>, need_cache: bool) -> Self {
        TaskForStore {
            instance,
            next_mission,
            previous_mission,
            need_cache,
        }
    }
}