extern crate r2d2;

use dao::thing::ThingDefineDao;
use define::*;
use serde_json;
use service::*;
use uuid::*;

/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Instance {
    /// Used to distinguish other instance
    pub id: UuidBytes,
    pub data: InstanceNoID,
}

impl Instance {
    fn verify(&mut self) -> Result<()> {
        let mut dao = DEFINE_DAO.lock().unwrap();
        let _def = dao.get(&self.data.thing);
        // TODO whether key configured

        Ok(())
    }

    fn id_generate_if_not_set(&mut self) -> Result<()> {
        let zero = self.id.into_iter().all(|x| *x == 0);
        if zero {
            let json = serde_json::to_string(&self.data)?;
            self.id = *Uuid::new_v3(&NAMESPACE_DNS, &json).as_bytes();
        }
        Ok(())
    }

    pub fn store(&mut self) -> Result<UuidBytes> {
        self.verify()?;
        // fill ID
        self.id_generate_if_not_set()?;
        // TODO

        Ok(self.id)
    }
}

