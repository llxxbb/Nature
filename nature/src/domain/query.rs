use crate::domain::*;
use crate::util::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Clone, Ord, PartialOrd)]
pub struct InsCond {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub id: u64,
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

impl InsCond {
    pub fn new(id: u64, meta: &str, para: &str, state_version: i32) -> Self {
        InsCond {
            id,
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
    fn get_id_str(&self) -> String {
        return if self.id == 0 { "".to_string() } else { self.id.to_string() };
    }
    pub fn id_like(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}%", self.meta, sep)
    }
    pub fn para_like(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}{}{}%", self.meta, sep, self.get_id_str(), sep)
    }
    pub fn get_key(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}{}{}{}", self.meta, sep, self.get_id_str(), sep, self.para)
    }
}

impl From<&Instance> for InsCond {
    fn from(input: &Instance) -> Self {
        InsCond {
            id: input.id,
            meta: input.path.meta.to_string(),
            key_gt: "".to_string(),
            key_ge: "".to_string(),
            key_lt: "".to_string(),
            key_le: "".to_string(),
            para: input.path.para.to_string(),
            state_version: input.path.state_version,
            time_ge: None,
            time_lt: None,
            limit: 1,
        }
    }
}

impl From<&InstanceLocator> for InsCond {
    fn from(input: &InstanceLocator) -> Self {
        InsCond {
            id: input.id,
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
mod ins_cond_test {
    use super::*;

    #[test]
    fn from_json() {
        let condition = r#"
            {
                "id": 1,
                "meta": "B:finance/payment:1",
                "para": "a",
                "state_version": 0
            }
        "#;
        let rtn = serde_json::from_str::<InsCond>(condition).unwrap();
        assert_eq!(rtn.id, 1)
    }

    #[test]
    #[ignore]
    fn to_json() {
        let condition = InsCond {
            id: 0,
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