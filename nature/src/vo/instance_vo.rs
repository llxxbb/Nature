use std::convert::TryInto;
use std::hash::Hash;

use chrono::{Local, NaiveDateTime, TimeZone};

use crate::domain::{BizObject, Instance, InstanceLocator, Modifier};
use crate::common::NatureError;
use crate::util::*;

/// A snapshot for a particular `Meta`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct InstanceVO {
    /// A unique value used to distinguish other instance
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub id: String,
    pub path: Modifier,
    /// data Nature can't controlled
    pub data: BizObject,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub from: Option<InstanceLocatorVO>,
    /// When this instance created in db
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub create_time: Option<NaiveDateTime>,
}

impl From<Instance> for InstanceVO {
    fn from(ins: Instance) -> Self {
        InstanceVO {
            id: ins.id.to_string(),
            path: ins.path.clone(),
            from: match &ins.from {
                None => None,
                Some(fm) => Some(fm.into())
            },
            create_time: Some(Local.timestamp_millis_opt(ins.create_time).unwrap().naive_local()),
            data: ins.data.clone(),
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
        let rtn = Instance {
            id,
            path: self.path.clone(),
            data: self.data.clone(),
            from,
            create_time: 0,
        };
        Ok(rtn)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InstanceLocatorVO {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub id: String,
    pub modifier: Modifier,
}

impl TryInto<InstanceLocator> for InstanceLocatorVO {
    type Error = NatureError;

    fn try_into(self) -> Result<InstanceLocator, Self::Error> {
        let id: u64 = self.id.parse()?;
        let rtn = InstanceLocator {
            id,
            modifier: self.modifier.clone(),
        };
        Ok(rtn)
    }
}

impl From<&InstanceLocator> for InstanceLocatorVO {
    fn from(fi: &InstanceLocator) -> Self {
        InstanceLocatorVO {
            id: fi.id.to_string(),
            modifier: fi.modifier.clone(),
        }
    }
}