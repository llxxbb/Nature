use db::trait_define::ThingDefineDaoTrait;
use diesel::prelude::*;
use super::*;

pub struct ThingDefineDaoImpl;

impl ThingDefineDaoTrait for ThingDefineDaoImpl {
    fn get(thing: &Thing) -> Result<Option<ThingDefine>> {
        use super::schema::thing_defines::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let def = thing_defines.filter(key.eq(&thing.key))
            .filter(version.eq(thing.version))
            .load::<ThingDefine>(conn)?;
        match def.len() {
            0 => Ok(None),
            1 => Ok(Some(def[0].clone())),
            _ => Err(NatureErrorWrapper::from(NatureError::SystemError("should less than 2 record return".to_string()))),
        }
    }

    fn insert(define: &ThingDefine) -> Result<usize> {
        use self::schema::thing_defines;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let rtn = diesel::insert_into(thing_defines::table)
            .values(NewThingDefine::new(define))
            .execute(conn);
        match rtn {
            Ok(x) => Ok(x),
            Err(e) => Err(NatureErrorWrapper::from(e))
        }
    }

    fn delete(thing: &Thing) -> Result<usize> {
        use self::schema::thing_defines::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let rtn = diesel::delete(thing_defines.filter(key.eq(&thing.key)).filter(version.eq(thing.version)))
            .execute(conn);
        match rtn {
            Ok(x) => Ok(x),
            Err(err) => Err(NatureErrorWrapper::from(err))
        }
    }
}