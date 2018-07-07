use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StoreInfo {
    pub instance: Instance,
    /// save outside has non converter info.
    pub converter: Option<ConverterInfo>,
}