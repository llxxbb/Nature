use data::*;
use uuid::UuidBytes;


#[derive(Debug)]
pub struct StorePlan {
    pub from_id: UuidBytes,
    pub to: Thing,
    pub plan: Vec<Instance>,
}