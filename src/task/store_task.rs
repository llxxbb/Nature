use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreTask(pub Instance);

unsafe impl Sync for StoreTask {}

unsafe impl Send for StoreTask {}

