use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConverterTask(pub Instance, pub Mapping);

unsafe impl Sync for ConverterTask {}

unsafe impl Send for ConverterTask {}
