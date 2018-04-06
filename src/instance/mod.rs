extern crate r2d2;

use dao::thing::ThingDefineDao;
use define::*;
use serde_json;
use service::*;
use thing::*;
use uuid::*;

/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Instance {
    /// Used to distinguish other instance
    pub id: UuidBytes,
    pub data: InstanceNoID,
}

impl Instance {
    pub fn verify(&mut self) -> Result<UuidBytes> {
        let mut dao = DEFINE_DAO.lock().unwrap();
        let _def = dao.get(&self.data.thing);
        // TODO whether key configured
        self.id_generate_if_not_set()
    }

    fn id_generate_if_not_set(&mut self) -> Result<UuidBytes> {
        let zero = self.id.into_iter().all(|x| *x == 0);
        if zero {
            let json = serde_json::to_string(&self.data)?;
            self.id = *Uuid::new_v3(&NAMESPACE_DNS, &json).as_bytes();
        }
        Ok(self.id)
    }

    pub fn store(&mut self) -> Result<()> {
        // TODO
        unimplemented!();
    }
}


/// A snapshot for a particular `Thing`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InstanceNoID {
    /// This instance's Type
    pub thing: Thing,
    /// The time which plan to flow for this instance
    pub execute_time: u64,
    /// When this instance created
    pub create_time: u64,
    /// What contend in this instance for the `Thing`
    pub content: String,
    /// Is a json for a `Map[key, value]` which contents other instance for other `Thing`'s.
    /// `Nature` can transform those to `Instance`'s by flowing.
    ///
    /// # Key
    ///
    /// context name
    ///
    /// # Value
    ///
    /// json data for a `Instance`.
    pub context: String,
}


#[cfg(test)]
mod tests;