use global::*;
use nature_common::*;
use std::marker::PhantomData;
use super::*;


pub trait PlanServiceTrait {
    fn new(converter_info: &ConverterInfo, instances: &Vec<Instance>) -> Result<PlanInfo>;
}

pub struct PlanServiceImpl<DAO> {
    dao: PhantomData<DAO>
}

impl<DAO> PlanServiceTrait for PlanServiceImpl<DAO> where DAO: StorePlanDaoTrait {
    fn new(converter_info: &ConverterInfo, instances: &Vec<Instance>) -> Result<PlanInfo> {
        let plan = PlanInfo {
            from_id: converter_info.from.id,
            from_thing: converter_info.from.thing.clone(),
            to: converter_info.target.to.clone(),
            plan: instances.clone(),
        };
        // reload old plan if exists
        match DAO::save(&plan) {
            Ok(plan) => Ok(plan),
            Err(err) => match err.err {
                NatureError::DaoDuplicated => DAO::get(&plan.from_id),
                _ => Err(err)
            },
        }
    }
}

