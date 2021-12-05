use crate::util::{is_default, str_2_64};

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Clone, Ord, PartialOrd)]
pub struct TaskCondition {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub task_for: String,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    #[serde(with = "str_2_64")]
    pub id_from: u64,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub limit: u32,
}