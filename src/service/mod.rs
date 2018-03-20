///! World Connection Service provider
extern crate uuid;

use define::Nature;
use define::ThingInstance;
use self::uuid::NAMESPACE_DNS;
use self::uuid::Uuid;
use self::uuid::UuidBytes;

pub struct Service;

impl Service {
    fn id_generate_if_not_set(&self, thing: ThingInstance) -> ThingInstance {
        let id = thing.id.into_iter().all(|x| *x == 0);
        if !id {
            // TODO
            let mut rtn = thing;
            rtn.id = *Uuid::new_v3(&NAMESPACE_DNS, "").as_bytes();
            return rtn;
        } else {
            return thing;
        }
    }
}

impl Nature for Service {
    fn flow(&self, thing: ThingInstance) -> Result<UuidBytes, String> {
        if thing.thing.key.is_empty() {
            return Err("[biz] must not be empty!".to_string());
        }

        let thing = self.id_generate_if_not_set(thing);

        let v3 = Uuid::new_v3(&NAMESPACE_DNS, "dfsadf");
        Ok(*v3.as_bytes())
    }
}

