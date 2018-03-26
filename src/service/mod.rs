///! World Connection Service provider
extern crate uuid;

use dao::instance::*;
use define::*;
use self::uuid::NAMESPACE_DNS;
use self::uuid::Uuid;
use self::uuid::UuidBytes;
use serde_json;
use serde_json::Error;
use dao::thing::THING_DAO;

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


impl Nature for NatureService {
    fn flow(&self, thing: Instance) -> Result<UuidBytes> {
        thing.verify(&THING_DAO)?;
        let thing = self.id_generate_if_not_set(thing)?;
        thing.store(&INSTANCE_DAO)
    }
}


#[cfg(test)]
mod test;
