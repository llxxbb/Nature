use dao::*;
use define::*;
use serde_json;
use uuid::*;

pub trait Nature<T: ThingDao<CONN>, I: InstanceDao> {
    fn flow(&self, thing: Instance, thing_dao: &T, instance_dao: &I) -> Result<UuidBytes>;
}

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

impl<T: ThingDao<CONN>, I: InstanceDao> Nature<T, I> for NatureService {
    fn flow(&self, thing: Instance, thing_dao: &T, instance_dao: &I) -> Result<UuidBytes> {
        thing.verify(thing_dao)?;
        let thing = self.id_generate_if_not_set(thing)?;
        thing.store(instance_dao)
    }
}
