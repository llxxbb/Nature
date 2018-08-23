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
        let mut plan = PlanInfo {
            from_sn: converter_info.from.id,
            from_thing: converter_info.from.thing.clone(),
            from_sta_ver: converter_info.from.status_version,
            to: converter_info.target.to.clone(),
            plan: instances.clone(),
        };
        // reload old plan if exists
        match DAO::save(&mut plan) {
            Ok(_) => Ok(plan),
            Err(err) => Err(err),
        }
    }
}

