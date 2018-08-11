use super::*;
use diesel::prelude::*;
use std::ops::Deref;
use util::id_tool::u128_to_vec_u8;

pub struct InstanceDaoImpl;

impl InstanceDaoTrait for InstanceDaoImpl {
    fn insert(instance: &Instance) -> Result<usize> {
        use self::schema::instances;
        let new = NewInstance::new(instance)?;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        diesel::insert_into(instances::table)
            .values(new)
            .execute(conn)
    }

    /// check whether source stored earlier
    fn is_exists(ins: &Instance) -> Result<bool> {
        use self::schema::instances::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let def = instances
            .filter(id.eq(ins.id.to_bytes().to_vec()))
            .filter(thing.eq(ins.thing.key.clone()))
            .filter(version.eq(ins.thing.version))
            .filter(status_version.eq(ins.status_version))
            .order(status_version.desc())
            .limit(1)
            .load::<NewInstance>(conn);
        match def {
            Ok(rs) => match rs.len() {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(Box::new(NatureError::SystemError("should less than 2 record return".to_string()))
            },
            Err(e) => Err(e)
        }
    }
    fn get_by_id(instance_id: u128) -> Result<Option<Instance>> {
        use self::schema::instances::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let def = instances
            .filter(id.eq(u128_to_vec_u8(instance_id)))
            .order(status_version.desc())
            .limit(1)
            .load::<NewInstance>(conn)?;
        match def.len() {
            0 => Ok(None),
            1 => Ok(Some(Instance::from(def[0].clone())?)),
            _ => Err(NatureError::SystemError(
                "should less than 2 record return".to_string(),
            )),
        }
    }
}

impl InstanceDaoImpl {
    pub fn delete(ins: &Instance) -> Result<usize> {
        debug!("delete instance : {:?}", ins);
        use self::schema::instances::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let rows = instances
            .filter(id.eq(ins.id.to_bytes().to_vec()))
            .filter(thing.eq(ins.thing.key.clone()))
            .filter(version.eq(ins.thing.version))
            .filter(status_version.eq(ins.status_version));
        //        debug!("rows filter : {:?}", rows);
        let rtn = diesel::delete(rows).execute(conn)?;
        Ok(rtn)
    }
}
