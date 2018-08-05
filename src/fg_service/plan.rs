use nature_common::*;
use std::marker::PhantomData;
use super::*;


/// **unique key**
/// * from_id
/// * from_thing
#[derive(Debug)]
pub struct PlanInfo {
    pub from_id: u128,
    pub from_thing: Thing,
    pub to: Thing,
    pub plan: Vec<Instance>,
}


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
            Err(NatureError::DaoDuplicated) => DAO::get(&plan.from_id),
            Err(err) => Err(err),
        }
    }
}

