use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::hash::Hash;

use crate::domain::{BizObject, FromInstance, Instance, NatureError};
use crate::util::*;
use crate::util::js_convert::try_to_i64;

/// A snapshot for a particular `Meta`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct InstanceVO {
    /// A unique value used to distinguish other instance
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub id: String,
    /// This instance's Type
    pub meta: String,
    /// What contend in this instance for the `Meta`
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub content: String,
    /// Is a json for a `Map[key, value]` which maybe used for next `Relation`
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub context: HashMap<String, String>,
    /// like `context` but is specified by Nature
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub sys_context: HashMap<String, String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub states: HashSet<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub state_version: i32,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub from: Option<FromInstanceVO>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub para: String,
    /// When this instance created in db
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub create_time: Option<String>,
}

impl From<Instance> for InstanceVO {
    fn from(ins: Instance) -> Self {
        InstanceVO {
            id: ins.id.to_string(),
            meta: ins.meta.to_string(),
            content: ins.content.to_string(),
            context: ins.context.clone(),
            sys_context: ins.sys_context.clone(),
            states: ins.states.clone(),
            state_version: ins.state_version,
            from: match &ins.from {
                None => None,
                Some(fm) => Some(fm.into())
            },
            para: ins.para.to_string(),
            create_time: Some(ins.create_time.to_string()),
        }
    }
}

impl TryInto<Instance> for InstanceVO {
    type Error = NatureError;

    fn try_into(self) -> Result<Instance, Self::Error> {
        let id: u64 = self.id.parse()?;
        let from = match self.from {
            None => None,
            Some(tar) => Some(tar.try_into()?)
        };
        let create_time = match self.create_time {
            None => 0 as i64,
            Some(_) => try_to_i64(self.create_time)?.unwrap()
        };
        let rtn = Instance {
            id,
            data: BizObject {
                meta: self.meta.to_string(),
                content: self.content.to_string(),
                context: self.context.clone(),
                sys_context: self.sys_context.clone(),
                states: self.states.clone(),
                state_version: self.state_version,
                from,
                para: self.para.to_string(),
            },
            create_time,
        };
        Ok(rtn)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct FromInstanceVO {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub id: String,
    pub meta: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub para: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub state_version: i32,
}

impl TryInto<FromInstance> for FromInstanceVO {
    type Error = NatureError;

    fn try_into(self) -> Result<FromInstance, Self::Error> {
        let id: u64 = self.id.parse()?;
        let rtn = FromInstance {
            id,
            meta: self.meta.to_string(),
            para: self.para.to_string(),
            state_version: self.state_version,
        };
        Ok(rtn)
    }
}

impl From<&FromInstance> for FromInstanceVO {
    fn from(fi: &FromInstance) -> Self {
        FromInstanceVO {
            id: fi.id.to_string(),
            meta: fi.meta.to_string(),
            para: fi.para.to_string(),
            state_version: fi.state_version,
        }
    }
}