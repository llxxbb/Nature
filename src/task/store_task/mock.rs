use super::*;

#[derive(Debug, Copy, Clone)]
pub enum StoreTaskMode {
    Ok,
    Err,
}
lazy_static! {
    pub static ref STORE_TASK_MODE: Mutex<StoreTaskMode> = Mutex::new(StoreTaskMode::Ok);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreTask(pub Instance);

impl Task for StoreTask {
    fn take_it_over(&self) -> Result<()> {
        let mode = &*STORE_TASK_MODE.lock().unwrap();
        println!("StoreTask mode is {:?}", mode);
        match mode {
            StoreTaskMode::Ok => Ok(()),
            StoreTaskMode::Err => Err(NatureError::StoreTaskError("some error".to_string()))
        }
    }
}
