use data::*;
use global::*;
use task::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct DispatchTask(pub Vec<Mapping>);

impl Task for DispatchTask {
    /// Persistent `Instance` to DB
    fn take_it_over(&self) -> Result<()> {
        // TODO dispatch task

        Ok(())
    }
}

unsafe impl Sync for DispatchTask {}

unsafe impl Send for DispatchTask {}