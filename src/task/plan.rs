use std::convert::{TryFrom, TryInto};

use chrono::Local;

use nature_common::*;
use nature_db::RawPlanInfo;

use crate::task::TaskForConvert;
use crate::lazy_static::__Deref;
use crate::system::PLAN_CONTENT_MAX_LENGTH;

/// **unique key**
/// * from_id
/// * from_thing
#[derive(Debug, Clone)]
pub struct PlanInfo {
    pub from_thing: BizMeta,
    pub from_sn: u128,
    pub from_sta_ver: i32,
    pub to: BizMeta,
    pub plan: Vec<Instance>,
}

impl PlanInfo {
    pub fn save<FI, FG>(converter_info: &TaskForConvert, instances: &Vec<Instance>, dao_insert: FI, dao_get: FG) -> Result<PlanInfo>
        where FI: Fn(&RawPlanInfo) -> Result<()>, FG: Fn(&str) -> Result<Option<RawPlanInfo>>
    {
        let plan = PlanInfo {
            from_sn: converter_info.from.id,
            from_thing: converter_info.from.thing.clone(),
            from_sta_ver: converter_info.from.status_version,
            to: converter_info.target.to.clone(),
            plan: instances.clone(),
        };

        // reload old plan if exists
        let will_save = plan.clone().try_into()?;
        match dao_insert(&will_save) {
            Ok(_) => Ok(plan),
            Err(err) => match err {
                NatureError::DaoDuplicated(msg) => {
                    let old = dao_get(&will_save.upstream)?;
                    match old {
                        Some(o) => PlanInfo::try_from(o),
                        None => Err(NatureError::SystemError(format!("old should exists for : {}", msg)))
                    }
                }
                _ => Err(err)
            }
        }
    }
}

impl TryFrom<RawPlanInfo> for PlanInfo {
    type Error = NatureError;

    fn try_from(value: RawPlanInfo) -> Result<Self> {
        let x: Vec<&str> = value.upstream.split(':').collect();
        if x.len() != 4 {
            return Err(NatureError::VerifyError("format error : ".to_owned() + &value.upstream));
        }
        Ok(PlanInfo {
            from_thing: BizMeta::from_full_key(x[0], x[1].parse()?)?,
            from_sn: x[2].parse()?,
            from_sta_ver: x[3].parse()?,
            to: BizMeta::from_full_key(&value.to_biz, value.to_version)?,
            plan: serde_json::from_str(&value.content)?,
        })
    }
}

impl TryInto<RawPlanInfo> for PlanInfo {
    type Error = NatureError;

    fn try_into(self) -> Result<RawPlanInfo> {
        let upstream = format!("{}:{}:{}:{}", self.from_thing.get_full_key(), self.from_thing.version, self.from_sn, self.from_sta_ver);
        Ok(RawPlanInfo {
            upstream,
            to_biz: self.to.get_full_key(),
            to_version: self.to.version,
            content: {
                let json = serde_json::to_string(&self.plan)?;
                if json.len() > *PLAN_CONTENT_MAX_LENGTH.deref() {
                    return Err(NatureError::DaoLogicalError("content's length can' be over : ".to_owned() + &PLAN_CONTENT_MAX_LENGTH.to_string()));
                }
                json
            },
            create_time: Local::now().naive_local(),
        })
    }
}

