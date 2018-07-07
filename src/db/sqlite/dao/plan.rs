use super::*;

pub struct StorePlanDaoService;

impl StorePlanDao for StorePlanDaoService {
    fn save(_plan: &PlanInfo) -> Result<PlanInfo> {
        unimplemented!()
    }

    fn get(_from_id: &u128) -> Result<PlanInfo> {
        unimplemented!()
    }
}