use db::*;
use global::*;
use super::*;
use task::ConverterInfo;

impl StorePlan {
    pub fn new(converter_info: &ConverterInfo, instances: &Vec<Instance>) -> Result<Self> {
        let plan = StorePlan {
            from_id: converter_info.from.id,
            from_thing: converter_info.mapping.from.clone(),
            to: converter_info.mapping.to.clone(),
            plan: instances.clone(),
        };
        // reload old plan if exists
        match StorePlanDaoService::save(&plan) {
            Ok(plan) => Ok(plan),
            Err(NatureError::DaoDuplicated) => StorePlanDaoService::get(&plan.from_id),
            Err(err) => Err(err),
        }
    }
}