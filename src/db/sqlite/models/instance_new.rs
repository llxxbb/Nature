use chrono::prelude::*;
use serde_json;
use super::*;
use super::schema::instances;

#[derive(Debug)]
#[derive(Insertable)]
#[table_name = "instances"]
pub struct NewInstance<'a> {
    id: Vec<u8>,
    thing: &'a str,
    version: i32,
    content: &'a str,
    context: Option<String>,
    status: Option<String>,
    status_version: i32,
    from_thing: Option<String>,
    from_version: Option<i32>,
    from_status_version: Option<i32>,
    execute_time: NaiveDateTime,
    create_time: NaiveDateTime,

}

impl<'a> NewInstance<'a> {
    pub fn new(instance: &'a Instance) -> Result<NewInstance> {
        let (from_thing, from_version, from_status_version) = match instance.from {
            None => (None, None, None),
            Some(ref from) => (Some(from.thing.key.clone()), Some(from.thing.version), Some(from.status_version))
        };
        Ok(NewInstance {
            id: instance.id.to_vec(),
            thing: &instance.thing.key,
            version: instance.thing.version,
            content: &instance.content,
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
            execute_time: NaiveDateTime::from_timestamp(instance.execute_time, 0),
            create_time: NaiveDateTime::from_timestamp(instance.create_time, 0),
        })
    }
}
