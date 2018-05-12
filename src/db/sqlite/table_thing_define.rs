use super::*;
use std::ops::Deref;
use diesel::prelude::*;

pub struct TableThingDefine;

impl TableThingDefine {
    pub fn get(thing: &Thing) -> Result<Option<ThingDefine>> {
        use self::schema::thing_defines::dsl::*;
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

    pub fn insert(define: &ThingDefine) -> Result<()> {
        use self::schema::thing_defines;
        let conn = DBPool::get_connection()?;
        // TODO
        let _ = diesel::insert_into(thing_defines::table)
            .values(NewThingDefine::new(define))
            .execute(conn.deref());
        Ok(())
    }

    pub fn delete(thing: &Thing) -> Result<()> {
        use self::schema::thing_defines::dsl::*;
        let conn = DBPool::get_connection()?;
        // TODO
        let _ = diesel::delete(thing_defines.filter(key.eq(&thing.key)).filter(version.eq(thing.version)))
            .execute(conn.deref());
        Ok(())
    }
}