use chrono::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
use super::*;
use super::schema::instances;

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name = "instances"]
pub struct NewInstance {
    id: Vec<u8>,
    thing: String,
    version: i32,
    content: String,
    context: Option<String>,
    status: Option<String>,
    status_version: i32,
    from_thing: Option<String>,
    from_version: Option<i32>,
    from_status_version: Option<i32>,
    event_time: NaiveDateTime,
    execute_time: NaiveDateTime,
    create_time: NaiveDateTime,
}

impl Instance {
    pub fn from(ni: NewInstance) -> Result<Self> {
        let from = match ni.from_thing {
            None => None,
            Some(k) => Some(FromInstance {
                thing: Thing {
                    key: k,
                    version: ni.from_version.unwrap(),
                },
                status_version: ni.from_status_version.unwrap(),
            })
        };
        let id = {
            let mut arr = [0u8; 16];
            for i in 0..16 {
                arr[i] = ni.id[i];
            }
            u128::from_bytes(arr)
        };
        let context = match ni.context {
            None => HashMap::new(),
            Some(ref s) => serde_json::from_str::<HashMap<String, String>>(s)?
        };
        let status = match ni.status {
            None => HashSet::new(),
            Some(ref s) => serde_json::from_str::<HashSet<String>>(s)?
        };
        Ok(Instance {
            id,
            data: InstanceNoID {
                thing: Thing {
                    key: ni.thing,
                    version: ni.version,
                },
                event_time: ni.event_time.timestamp_millis(),
                execute_time: ni.execute_time.timestamp_millis(),
                create_time: ni.create_time.timestamp_millis(),
                content: ni.content,
                context,
                status,
                status_version: ni.status_version,
                from,
            },
        })
    }
}

impl NewInstance {
    pub fn new(instance: &Instance) -> Result<NewInstance> {
        let (from_thing, from_version, from_status_version) = match instance.from {
            None => (None, None, None),
            Some(ref from) => (Some(from.thing.key.clone()), Some(from.thing.version), Some(from.status_version))
        };
        Ok(NewInstance {
            id: instance.id.to_bytes().to_vec(),
            thing: instance.thing.key.clone(),
            version: instance.thing.version,
            content: instance.content.clone(),
            context: match instance.content.len() {
                0 => None,
                _ => Some(serde_json::to_string(&instance.context)?)
            },
            status: match instance.status.len() {
                0 => None,
                _ => Some(serde_json::to_string(&instance.status)?)
            },
            status_version: instance.status_version,
            from_thing,
            from_version,
            from_status_version,
            event_time: NaiveDateTime::from_timestamp(instance.event_time, 0),
            execute_time: NaiveDateTime::from_timestamp(instance.execute_time, 0),
            create_time: NaiveDateTime::from_timestamp(instance.create_time, 0),
        })
    }
}
