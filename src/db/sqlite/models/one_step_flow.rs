use data::*;
use global::*;
use nature_common::*;
use serde_json;
use std::str::FromStr;
use super::super::schema::one_step_flow;

#[derive(Debug)]
#[derive(Insertable, Queryable)]
#[table_name = "one_step_flow"]
pub struct OneStepFlowRow {
    pub from_thing: String,
    pub from_version: i32,
    pub to_thing: String,
    pub to_version: i32,
    pub exe_protocol: String,
    pub exe_url: String,
    pub selector: Option<String>,
    pub group: Option<String>,
    pub weight: Option<i32>,
}

impl OneStepFlow {
    pub fn from_row(val: OneStepFlowRow) -> Result<OneStepFlow> {
        let selector = match val.selector {
            None => None,
            Some(x) => {
                let opt = serde_json::from_str::<Selector>(&x);
                if let Err(e) = &opt {
                    warn!("{:?}", e);
                }
                let s = opt?;
                Some(s)
            }
        };
        let weight = match val.group {
            None => None,
            Some(x) => {
                let w = Weight {
                    label: x,
                    proportion: {
                        match val.weight {
                            None => 1 as i32,
                            Some(y) => y
                        }
                    },
                };
                Some(w)
            }
        };
        let rtn = OneStepFlow {
            from: Thing {
                key: val.from_thing,
                version: val.from_version,
                thing_type: ThingType::Business,
            },
            to: Thing {
                key: val.to_thing,
                version: val.to_version,
                thing_type: ThingType::Business,
            },
            executor: Executor {
                protocol: Protocol::from_str(&val.exe_protocol)?,
                url: val.exe_url,
            },
            selector,
            weight,
        };
        Ok(rtn)
    }
}