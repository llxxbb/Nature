///! World Connection Service provider
extern crate uuid;

use define::Instance;
use define::Nature;
use self::uuid::NAMESPACE_DNS;
use self::uuid::Uuid;
use self::uuid::UuidBytes;
use serde_json;
use serde_json::Error;

pub struct Service;

impl Service {
    fn id_generate_if_not_set(&self, thing: Instance) -> Result<Instance, Error> {
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

impl Nature for Service {
    fn flow(&self, thing: Instance) -> Result<UuidBytes, String> {
        if thing.data.thing.key.is_empty() {
            return Err("[biz] must not be empty!".to_string());
        }

        let thing = self.id_generate_if_not_set(thing);

        let v3 = Uuid::new_v3(&NAMESPACE_DNS, "dfsadf");
        Ok(*v3.as_bytes())
    }
}

#[cfg(test)]
mod test;
