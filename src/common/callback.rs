use crate::common::ConverterReturned;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DelayedInstances {
    pub task_id: u64,
    pub result: ConverterReturned,
}
