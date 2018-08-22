use data::PlanInfo;
use diesel::prelude::*;
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