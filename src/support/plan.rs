use nature_common::{Instance, NatureError, Result};
use nature_db::{PlanInfo, RawPlanInfo};

use crate::flow::ConverterInfo;

pub struct PlanInfoSvc;

impl PlanInfoSvc {
    pub fn save<FI, FG>(converter_info: &ConverterInfo, instances: &Vec<Instance>, dao_insert: FI, dao_get: FG) -> Result<PlanInfo>
        where FI: Fn(&RawPlanInfo) -> Result<()>, FG: Fn(&str) -> Result<Option<PlanInfo>>
    {
        let plan = PlanInfo {
            from_sn: converter_info.from.id,
            from_thing: converter_info.from.thing.clone(),
            from_sta_ver: converter_info.from.status_version,
            to: converter_info.target.to.clone(),
            plan: instances.clone(),
        };

        // reload old plan if exists
        let will_save = RawPlanInfo::new(&plan)?;
        match dao_insert(&will_save) {
            Ok(_) => Ok(plan),
            Err(err) => match err {
                NatureError::DaoDuplicated(msg) => {
                    let old = dao_get(&will_save.upstream)?;
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