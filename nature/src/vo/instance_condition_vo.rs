use std::prelude::rust_2021::TryInto;

use crate::domain::{InsCond, NatureError, NoIdCond};
use crate::util::is_default;
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
    pub time_ge: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub time_lt: Option<String>,
    pub other: NoIdCond,
}

impl TryInto<InsCond> for InsCondVO {
    type Error = NatureError;

    fn try_into(self) -> Result<InsCond, Self::Error> {
        let id: u64 = if self.id.is_empty() { 0 } else { self.id.parse()? };
        let time_lt: Option<i64> = try_to_i64(self.time_lt)?;
        let time_ge: Option<i64> = try_to_i64(self.time_ge)?;
        let rtn = InsCond {
            id,
            time_ge,
            time_lt,
            other: self.other,
        };
        Ok(rtn)
    }
}

impl From<InsCond> for InsCondVO {
    fn from(input: InsCond) -> Self {
        InsCondVO {
            id: input.id.to_string(),
            time_ge: match input.time_ge {
                None => None,
                Some(t) => Some(t.to_string())
            },
            time_lt: match input.time_lt {
                None => None,
                Some(t) => Some(t.to_string())
            },
            other: input.other,
        }
    }
}