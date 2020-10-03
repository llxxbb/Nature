use std::collections::HashSet;

use crate::common::is_default;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct LastSelector {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub last_all: HashSet<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub last_any: HashSet<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub last_none: HashSet<String>,
}