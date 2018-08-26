use chrono::prelude::*;
use data::*;
use global::*;
use serde_json;
use super::super::schema::plan;

#[derive(Debug)]
#[derive(Insertable, Queryable)]
#[table_name = "plan"]
pub struct RawPlanInfo {
    pub upstream: String,
    pub to_biz: String,
    pub to_version: i32,
    pub content: String,
    create_time: NaiveDateTime,
}

impl RawPlanInfo {
    pub fn new(plan: &PlanInfo) -> Result<RawPlanInfo> {
        let upstream = format!("{}:{}:{}:{}", plan.from_thing.key, plan.from_thing.version, plan.from_sn, plan.from_sta_ver);
        Ok(RawPlanInfo {
            upstream,
            to_biz: plan.to.key.clone(),
            to_version: plan.to.version,
            content: serde_json::to_string(&plan.plan)?,
            create_time: Local::now().naive_local(),
        })
    }
    pub fn to_plan_info(&self) -> Result<PlanInfo> {
        let x: Vec<&str> = self.upstream.split(":").collect();
        if x.len() != 4 {
            return Err(NatureErrorWrapper::from(NatureError::VerifyError("format error : ".to_owned() + &self.upstream)));
        }
        Ok(PlanInfo {
            from_thing: Thing {
                key: x[0].to_string(),
                version: x[1].parse()?,
                thing_type: ThingType::Business,
            },
            from_sn: x[2].parse()?,
            from_sta_ver: x[3].parse()?,
            to: Thing {
                key: self.to_biz.clone(),
                version: self.to_version,
                thing_type: ThingType::Business,
            },
            plan: serde_json::from_str(&self.content)?,
        })
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn to_plan_info(){
        RawPlanInfo
    }
}
