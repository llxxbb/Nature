use nature_common::Instance;
use nature_db::Mission;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConverterInfo {
    pub from: Instance,
    pub target: Mission,
    pub last_status: Option<Instance>,
}

impl Default for ConverterInfo {
    fn default() -> Self {
        ConverterInfo {
            from: Instance::default(),
            target: Mission::default(),
            last_status: None,
        }
    }
}
