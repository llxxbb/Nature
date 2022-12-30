use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use crate::common::*;

use crate::domain::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InstanceLocator {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub id: u64,
    pub modifier: Modifier,
}

impl Deref for InstanceLocator {
    type Target = Modifier;

    fn deref(&self) -> &Self::Target {
        &self.modifier
    }
}

impl DerefMut for InstanceLocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.modifier
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Modifier {
    /// This instance's Type
    pub meta: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub para: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub state_version: i32,
}

impl InstanceLocator {
    pub fn from_key_no_state(key: &str) -> Result<Self> {
        let part: Vec<&str> = key.split(&*SEPARATOR_INS_KEY).collect();
        if part.len() != 3 {
            return Err(NatureError::VerifyError("format error".to_string()));
        }
        let rtn = InstanceLocator {
            id: part[1].parse()?,
            modifier: Modifier {
                meta: part[0].to_string(),
                para: part[2].to_string(),
                state_version: 0,
            },
        };
        Ok(rtn)
    }
    fn get_id_str(&self) -> String {
        return if self.id == 0 { "".to_string() } else { self.id.to_string() };
    }
}

impl From<&Instance> for InstanceLocator {
    fn from(from: &Instance) -> Self {
        InstanceLocator {
            id: from.id,
            modifier: Modifier {
                meta: from.path.meta.to_string(),
                para: from.path.para.clone(),
                state_version: from.path.state_version,
            },
        }
    }
}

impl FromStr for InstanceLocator {
    type Err = NatureError;

    fn from_str(s: &str) -> Result<Self> {
        let part: Vec<&str> = s.split(&*SEPARATOR_INS_KEY).collect();
        if part.len() != 4 {
            let msg = format!("FromInstance::from_str : error input [{}]", s);
            return Err(NatureError::VerifyError(msg));
        }
        let id: u64 = if part[1].len() == 0 { 0 } else { part[1].parse()? };
        let rtn = InstanceLocator {
            id,
            modifier: Modifier {
                meta: part[0].to_string(),
                para: part[2].to_string(),
                state_version: i32::from_str(part[3])?,
            },
        };
        Ok(rtn)
    }
}

impl ToString for InstanceLocator {
    fn to_string(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}{}{}{}{}{}", self.meta, sep, self.get_id_str(), sep, self.para, sep, self.state_version)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_string_test() {
        let from = InstanceLocator::default();
        let string = from.to_string();
        assert_eq!(string, "|||0");
        let rtn = InstanceLocator::from_str(&string).unwrap();
        assert_eq!(from, rtn);
    }
}