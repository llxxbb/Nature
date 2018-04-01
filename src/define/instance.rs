extern crate r2d2;

use dao::*;
use define::*;
use self::r2d2::{ManageConnection, Pool};
use uuid::UuidBytes;

/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Instance {
    /// Used to distinguish other instance
    pub id: UuidBytes,
    pub data: InstanceNoID,
}

impl Instance {
    pub fn verify<T: ThingDao<CONN>>(&mut self, dao: &T, conn: Pool<CONN>) -> Result<()> {
        if self.data.thing.key.is_empty() {
            return Err(NatureError::VerifyError("[biz] must not be empty!".to_string()));
        }
        dao.get(&self.data.thing, conn);
        // TODO whether key configured
        Ok(())
    }
}