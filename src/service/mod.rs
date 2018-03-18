///! World Connection Service provider
extern crate uuid;

use define::ThingInstance;
use define::Nature;
use self::uuid::Uuid;
use self::uuid::NAMESPACE_DNS;
pub struct Service;


impl Nature for Service {
    fn transform(&self, data: ThingInstance) -> Result<[u8;16], String> {
        if data.thing.id.is_empty() {
            return Err("[biz] must not be empty!".to_string());
        }
        let v3 = Uuid::new_v3(&NAMESPACE_DNS, "dfsadf");
        Ok(*v3.as_bytes())
    }
}

