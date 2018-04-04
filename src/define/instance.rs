extern crate r2d2;

use define::*;
use service::*;
use uuid::UuidBytes;
use dao::thing::ThingDefineDao;

/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Instance {
    /// Used to distinguish other instance
    pub id: UuidBytes,
    pub data: InstanceNoID,
}

impl Instance {
    pub fn verify(&self) -> Result<()> {
        let mut dao = DEFINE_DAO.lock().unwrap();
        let _def = dao.get(&self.data.thing);
        // TODO whether key configured
        Ok(())
    }
}