use std::sync::mpsc::Sender;

use nature_common::{DynamicConverter, Instance, Meta, Result};
use nature_db::{Mission, OneStepFlow, RawTask, RelationGetter, RelationResult};

use crate::task::TaskForConvert;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskForStore {
    pub instance: Instance,
    /// save outside has non converter info.
    pub upstream: Option<TaskForConvert>,
    pub mission: Option<Vec<Mission>>,
}


impl TaskForStore {
    pub fn gen_task<FF>(instance: &Instance, step_cache_getter: fn(&Meta, RelationGetter) -> RelationResult, relation_getter: RelationGetter, mission_filter: FF) -> Result<Self> where
        FF: FnOnce((&Instance, Vec<OneStepFlow>)) -> Option<Vec<Mission>>
    {
        let steps = match step_cache_getter(&instance.meta, relation_getter)? {
            Some(steps) => {
                mission_filter((&instance, steps))
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