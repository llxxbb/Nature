use db::trait_define::InstanceDaoTrait;
use diesel::prelude::*;
use super::*;
use util::id_tool::u128_to_vec_u8;

pub struct InstanceDaoImpl;

impl InstanceDaoTrait for InstanceDaoImpl {
    fn insert(instance: &Instance) -> Result<usize> {
        use self::schema::instances;
        let new = NewInstance::new(instance)?;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        match diesel::insert_into(instances::table)
            .values(new)
            .execute(conn) {
            Ok(rtn) => Ok(rtn),
            Err(err) => Err(NatureErrorWrapper::from(err))
        }
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
                _ => Err(NatureErrorWrapper::from(NatureError::SystemError("should less than 2 record return".to_string())))
            },
            Err(e) => Err(NatureErrorWrapper::from(e))
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
            1 => Ok(Some(def[0].to()?)),
            _ => Err(NatureErrorWrapper::from(NatureError::SystemError("should less than 2 record return".to_string(),))),
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
