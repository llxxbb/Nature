use db::trait_define::StorePlanDaoTrait;
use diesel::prelude::*;
use fg_service::PlanInfo;
use super::*;

pub struct StorePlanDaoImpl;

impl StorePlanDaoTrait for StorePlanDaoImpl {
    fn save(_plan: &PlanInfo) -> Result<PlanInfo> {
//        use self::schema::
        unimplemented!()
    }

    fn get(_from_id: &u128) -> Result<PlanInfo> {
        unimplemented!()
    }
}