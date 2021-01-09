use crate::util::*;

/// used for converter setting
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TargetState {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub remove: Option<Vec<String>>,
}
