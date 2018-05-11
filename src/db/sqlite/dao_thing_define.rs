use db::schema::thing_defines::dsl::*;
use diesel::prelude::*;
use global::*;
use std::ops::Deref;
use super::*;

pub struct TableThingDefine;

// #[derive(Debug)]
// #[derive(Insertable)]
// #[table_name="thing_defines"]
// struct NewThingDefine{      

// }

impl TableThingDefine {
    pub fn get(thing: &Thing) -> Result<Option<ThingDefine>> {
        let conn = DBPool::get_connection()?;
        let def = thing_defines.filter(key.eq(&thing.key))
            .filter(version.eq(thing.version))
            .load::<ThingDefine>(conn.deref())?;
        match def.len() {
            0 => Ok(None),
            1 => Ok(Some(def[0].clone())),
            _ => Err(NatureError::SystemError("should less than 2 record return".to_string())),
        }
    }

    fn insert(define: &ThingDefine) -> Result<()> {
        let conn = DBPool::get_connection()?;
        // TODO
        // diesel::insert_into(thing_defines)
        //     .values(define)
        //     .get_result(conn.deref())
        Ok(())
    }
}