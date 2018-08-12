use db::trait_define::StorePlanDaoTrait;
use fg_service::PlanInfo;
use super::*;

pub struct StorePlanDaoImpl;

impl StorePlanDaoTrait for StorePlanDaoImpl {
    fn save(_plan: &PlanInfo) -> Result<PlanInfo> {
        unimplemented!()
    }

    fn get(_from_id: &u128) -> Result<PlanInfo> {
        unimplemented!()
    }
}