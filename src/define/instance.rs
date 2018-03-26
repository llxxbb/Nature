use dao::instance::InstanceDao;
use define::*;
use uuid::UuidBytes;

/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Instance {
    /// Used to distinguish other instance
    pub id: UuidBytes,
    pub data: InstanceNoID,
}

impl Instance {
    pub fn verify<T: InstanceDao>(&self, dao: &T) -> Result<()> {
        if self.data.thing.key.is_empty() {
            return Err(NatureError::VerifyError("[biz] must not be empty!".to_string()));
        }
        // TODO whether key configured
        Ok(())
    }
}