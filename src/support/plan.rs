use std::rc::Rc;

use nature_common::*;

use super::*;

pub trait PlanServiceTrait {
    #[allow(clippy::ptr_arg)]
    fn save_plan(&self, converter_info: &ConverterInfo, instances: &Vec<Instance>) -> Result<PlanInfo>;
}

pub struct PlanServiceImpl {
    pub dao: Rc<StorePlanDaoTrait>
}

impl PlanServiceTrait for PlanServiceImpl {
    #[allow(clippy::ptr_arg)]
    fn save_plan(&self, converter_info: &ConverterInfo, instances: &Vec<Instance>) -> Result<PlanInfo> {
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

