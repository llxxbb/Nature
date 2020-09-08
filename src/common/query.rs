use crate::common::{FromInstance, ID, Instance, is_default, is_one, one, SEPARATOR_INS_KEY};

/// used for query instance by id
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KeyCondition {
    pub id: String,
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
        format!("{}{}{}{}%", self.meta, sep, self.id, sep)
    }
    pub fn get_key(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}{}{}{}", self.meta, sep, self.id, sep, self.para)
    }
}

impl From<&Instance> for KeyCondition {
    fn from(input: &Instance) -> Self {
        KeyCondition {
            id: format!("{:x}", input.id),
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
            id: format!("{:x}", input.id),
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
    pub id: ID,
    pub meta: String,
    pub from_key: String,
}

impl IDAndFrom {
    pub fn para_like(&self) -> String {
        format!("{}|{:x}|%", self.meta, self.id)
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
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn key_condition_test() {
        let condition = KeyCondition {
            id: "$id".to_string(),
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