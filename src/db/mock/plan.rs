use super::*;

pub struct StorePlanDaoService;

impl StorePlanDao for StorePlanDaoService{
    fn save(_plan: &StorePlan) -> Result<StorePlan> {
        unimplemented!()
    }

    fn get(_from_id: &[u8; 16]) -> Result<StorePlan> {
        unimplemented!()
    }
}