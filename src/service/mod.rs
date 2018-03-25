///! World Connection Service provider
extern crate uuid;

use dao::instance::InstanceDaoService;
use define::*;
use define::error::NatureError;
use self::uuid::NAMESPACE_DNS;
use self::uuid::Uuid;
use self::uuid::UuidBytes;
use serde_json;
use serde_json::Error;

pub struct NatureService;

impl NatureService {
    fn id_generate_if_not_set(&self, thing: Instance) -> Result<Instance> {
        let zero = thing.id.into_iter().all(|x| *x == 0);
        if zero {
            let mut rtn = thing;
            let json = serde_json::to_string(&rtn.data)?;
            rtn.id = *Uuid::new_v3(&NAMESPACE_DNS, &json).as_bytes();
            return Ok(rtn);
        } else {
            return Ok(thing);
        }
    }
}

static INSTANCE_DAO: InstanceDaoService = InstanceDaoService;

impl Nature for NatureService {
    fn flow(&self, thing: Instance) -> Result<UuidBytes> {
        if thing.data.thing.key.is_empty() {
            return Err(NatureError::VerifyError("[biz] must not be empty!".to_string()));
        }

        let thing = self.id_generate_if_not_set(thing)?;

        thing.store(&INSTANCE_DAO);

        let v3 = Uuid::new_v3(&NAMESPACE_DNS, "dfsadf");
        Ok(*v3.as_bytes())
    }
}


#[cfg(test)]
mod test;
