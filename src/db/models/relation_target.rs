use crate::common::{is_default, TargetState};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct RelationTarget {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub states: Option<TargetState>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub append_para: Vec<u8>,
    /// put `append_para` pointed value to `sys_context`'s `para.dynamic` property
    /// this is the key of the para.dynamic
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub context_name: String,
}