use dao::*;
use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreTask(pub Instance);

impl Task for StoreTask {
    /// Persistent `Instance` to DB
    fn take_it_over(&self) -> Result<()> {
        // TODO
        InstanceDaoService::insert(&self.0)?;

        // TODO process serially

        Ok(())
    }
}

unsafe impl Sync for StoreTask {}

unsafe impl Send for StoreTask {}