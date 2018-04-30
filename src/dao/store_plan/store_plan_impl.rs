use super::*;

pub struct StorePlanDaoService;

impl StorePlanDao for StorePlanDaoService {
    fn save(_plan: &StorePlan) -> Result<StorePlan> {
        unimplemented!()
    }
    fn get(_from_id: &UuidBytes) -> Result<StorePlan> {
        unimplemented!()
    }
}


