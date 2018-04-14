use super::*;

#[derive(Debug)]
pub struct StoreTask(pub Instance);

impl Task for StoreTask {
    fn take_it_over(&self) -> Result<()> {
        // TODO
        InstanceDaoService::insert(&self.0)?;

        // TODO process serially

        Ok(())
    }
}

unsafe impl Sync for StoreTask {}

unsafe impl Send for StoreTask {}