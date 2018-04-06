use instance::*;
use super::*;

#[derive(Debug)]
pub struct StoreTask(pub Instance);

impl Task for StoreTask {
    fn take_it_over(&self) -> Result<()> {
        // TODO
//        unimplemented!()
        Ok(())
    }
}

unsafe impl Sync for StoreTask{}

unsafe impl Send for StoreTask{}