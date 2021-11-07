use std::prelude::rust_2021::TryInto;

use crate::domain::{InsCond, NatureError};
use crate::util::{is_default, is_one, one};
use crate::util::js_convert::try_to_i64;

/// Condition for querying multi-row of `Instance`
/// key format [meta|id|para|status_version]
/// gt: grate than, only valid on the last part of the [key]
/// ge: grate than or equal, only valid on the last part of the [key]
/// lt: less than, only valid on the last part of the [key]
/// le: less than or equal, only valid on the last part of the [key]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InsCondVO {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub meta: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub key_gt: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub key_ge: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub key_lt: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub key_le: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub para: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub state_version: i32,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub time_ge: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub time_lt: Option<String>,
    #[serde(skip_serializing_if = "is_one")]
    #[serde(default = "one")]
    pub limit: i32,
}

impl TryInto<InsCond> for InsCondVO {
    type Error = NatureError;

    fn try_into(self) -> Result<InsCond, Self::Error> {
        let id: u64 = self.id.parse()?;
        let time_lt: Option<i64> = try_to_i64(self.time_lt)?;
        let time_ge: Option<i64> = try_to_i64(self.time_ge)?;
        let rtn = InsCond {
            id,
            meta: self.meta.to_string(),
            key_gt: self.key_gt.to_string(),
            key_ge: self.key_ge.to_string(),
            key_lt: self.key_lt.to_string(),
            key_le: self.key_le.to_string(),
            para: self.para.to_string(),
            state_version: self.state_version,
            time_ge,
            time_lt,
            limit: self.limit,
        };
        Ok(rtn)
    }
}

impl From<InsCond> for InsCondVO {
    fn from(input: InsCond) -> Self {
        InsCondVO {
            id: input.id.to_string(),
            meta: input.meta.to_string(),
            key_gt: input.key_gt.to_string(),
            key_ge: input.key_ge.to_string(),
            key_lt: input.key_lt.to_string(),
            key_le: input.key_le.to_string(),
            para: input.para.to_string(),
            state_version: input.state_version,
            time_ge: match input.time_ge {
                None => None,
                Some(t) => Some(t.to_string())
            },
            time_lt: match input.time_lt {
                None => None,
                Some(t) => Some(t.to_string())
            },
            limit: input.limit,
        }
    }
}