use nature_common::{Instance, Result, Thing};
use nature_db::{ConverterInfo, Mission, OneStepFlow, RawTask};
use std::sync::mpsc::Sender;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StoreTaskInfo {
    pub instance: Instance,
    /// save outside has non converter info.
    pub upstream: Option<ConverterInfo>,
    pub mission: Option<Vec<Mission>>,
}

impl StoreTaskInfo {
    pub fn gen_task<FG, FF>(instance: &Instance, step_getter: FG, mission_filter: FF) -> Result<Self> where
        FG: Fn(&Thing) -> Result<Option<Vec<OneStepFlow>>>,
        FF: FnOnce((&Instance, Vec<OneStepFlow>)) -> Option<Vec<Mission>>
    {
        let steps = match step_getter(&instance.thing)? {
            Some(steps) => {
                mission_filter((&instance, steps))
            }
            None => None
        };
        Ok(
            StoreTaskInfo {
                instance: instance.clone(),
                upstream: None,
                mission: steps,
            }
        )
    }

    pub fn send(&self, raw: &RawTask, sender: &Sender<(StoreTaskInfo,RawTask)>){
        let _ = sender.send((self.to_owned(),raw.to_owned()));
    }
}