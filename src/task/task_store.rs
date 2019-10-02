use std::sync::mpsc::Sender;

use nature_common::{DynamicConverter, Instance, Result};
use nature_db::{Mission, RawTask};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskForStore {
    pub instance: Instance,
    pub mission: Option<Vec<Mission>>,
}

impl TaskForStore {
    pub fn send(&self, raw: &RawTask, sender: &Sender<(TaskForStore, RawTask)>) {
        let _ = sender.send((self.to_owned(), raw.to_owned()));
    }

    pub fn for_dynamic(instance: &Instance, dynamic: Vec<DynamicConverter>) -> Result<TaskForStore> {
        let target = Mission::for_dynamic(dynamic)?;
        // save to task to make it can redo
        let task = TaskForStore {
            instance: instance.clone(),
            mission: Some(target),
        };
        Ok(task)
    }
}