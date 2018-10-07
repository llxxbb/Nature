use nature_common::*;
use std::rc::Rc;
use super::*;


pub trait PlanServiceTrait {
    fn new(&self, converter_info: &ConverterInfo, instances: &Vec<Instance>) -> Result<PlanInfo>;
}

pub struct PlanServiceImpl {
    dao: Rc<StorePlanDaoTrait>
}

impl PlanServiceTrait for PlanServiceImpl {
    fn new(&self, converter_info: &ConverterInfo, instances: &Vec<Instance>) -> Result<PlanInfo> {
        let plan = PlanInfo {
            from_sn: converter_info.from.id,
            from_thing: converter_info.from.thing.clone(),
            from_sta_ver: converter_info.from.status_version,
            to: converter_info.target.to.clone(),
            plan: instances.clone(),
        };
        // reload old plan if exists
        let will_save = RawPlanInfo::new(&plan)?;

        match self.dao.save(&will_save) {
            Ok(_) => Ok(plan),
            Err(err) => match err {
                NatureError::DaoDuplicated(msg) => {
                    let old = self.dao.get(&will_save.upstream)?;
                    match old {
                        Some(o) => {
                            Ok(o)
                        }
                        None => Err(NatureError::SystemError(format!("old should exists for : {}", msg)))
                    }
                }
                _ => Err(err)
            }
        }
    }
}

