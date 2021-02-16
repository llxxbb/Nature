use crate::domain::*;
use crate::util::*;
use std::str::FromStr;

/// Condition for querying multi-row of `Instance`
/// key format [meta|id|para|status_version]
/// gt: grate than, only valid on the last part of the [key]
/// ge: grate than or equal, only valid on the last part of the [key]
/// lt: less than, only valid on the last part of the [key]
/// le: less than or equal, only valid on the last part of the [key]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KeyCondition {
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
    pub time_ge: Option<i64>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub time_lt: Option<i64>,
    #[serde(skip_serializing_if = "is_one")]
    #[serde(default = "one")]
    pub limit: i32,
}

impl KeyCondition {
    pub fn new(id: &str, meta: &str, para: &str, state_version: i32) -> Self {
        KeyCondition {
            id: id.to_string(),
            meta: meta.to_string(),
            key_gt: "".to_string(),
            key_ge: "".to_string(),
            key_lt: "".to_string(),
            key_le: "".to_string(),
            para: para.to_string(),
            state_version,
            time_ge: None,
            time_lt: None,
            limit: 1,
        }
    }
    pub fn id_like(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}%", self.meta, sep)
    }
    pub fn para_like(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        let id = if self.id == "0" { "" } else { &self.id };
        format!("{}{}{}{}%", self.meta, sep, id, sep)
    }
    pub fn get_key(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        let id = if self.id == "0" { "" } else { &self.id };
        format!("{}{}{}{}{}", self.meta, sep, id, sep, self.para)
    }
    pub fn get_id(&self) -> Result<u64> {
        if self.id.is_empty() { Ok(0) } else { Ok(u64::from_str(&self.id)?) }
    }
}

impl From<&Instance> for KeyCondition {
    fn from(input: &Instance) -> Self {
        KeyCondition {
            id: input.id.to_string(),
            meta: input.meta.to_string(),
            key_gt: "".to_string(),
            key_ge: "".to_string(),
            key_lt: "".to_string(),
            key_le: "".to_string(),
            para: input.para.to_string(),
            state_version: input.state_version,
            time_ge: None,
            time_lt: None,
            limit: 1,
        }
    }
}

impl From<&FromInstance> for KeyCondition {
    fn from(input: &FromInstance) -> Self {
        KeyCondition {
            id: input.id.to_string(),
            meta: input.meta.to_string(),
            key_gt: "".to_string(),
            key_ge: "".to_string(),
            key_lt: "".to_string(),
            key_le: "".to_string(),
            para: input.para.to_string(),
            state_version: input.state_version,
            time_ge: None,
            time_lt: None,
            limit: 1,
        }
    }
}

/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IDAndFrom {
    pub id: u64,
    pub meta: String,
    pub from_key: String,
}

impl IDAndFrom {
    pub fn para_like(&self) -> String {
        format!("{}|{}|%", self.meta, self.id)
    }
}

/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QueryByMeta {
    pub meta: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub para_like: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub create_time_gt: Option<i64>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub create_time_ge: Option<i64>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub create_time_desc: bool,

}

#[cfg(test)]
mod key_condition_test {
    use super::*;

    #[test]
    fn from_json() {
        let condition = r#"
            {
                "id": "1",
                "meta": "B:finance/payment:1",
                "para": "a",
                "state_version": 0
            }
        "#;
        let rtn = serde_json::from_str::<KeyCondition>(condition).unwrap();
        assert_eq!(rtn.id, "1")
    }

    #[test]
    #[ignore]
    fn to_json() {
        let condition = KeyCondition {
            id: "0".to_string(),
            meta: "$meta".to_string(),
            key_gt: "".to_string(),
            key_ge: "".to_string(),
            key_lt: "".to_string(),
            key_le: "".to_string(),
            para: "".to_string(),
            state_version: 1,
            time_ge: None,
            time_lt: None,
            limit: 1,
        };
        let result = serde_json::to_string(&condition).unwrap();
        dbg!(result);
    }
}