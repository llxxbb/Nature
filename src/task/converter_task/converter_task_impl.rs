use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConverterTask(pub Instance, pub Mapping);

impl Task for ConverterTask {
    /// Persistent `Instance` to DB
    fn take_it_over(&self) -> Result<()> {
        // TODO call out

        Ok(())
    }
}

unsafe impl Sync for ConverterTask {}

unsafe impl Send for ConverterTask {}