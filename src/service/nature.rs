use define::*;
use serde_json;
use uuid::*;

pub trait Nature {
    fn flow(&self, thing: Instance) -> Result<UuidBytes>;
}

pub struct NatureService;

impl Nature for NatureService {
    fn flow(&self, thing: Instance) -> Result<UuidBytes> {
        thing.verify()?;
        let thing = self.id_generate_if_not_set(thing)?;
        thing.store()
    }
}

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
