use diesel::prelude::*;
use super::*;
use util::id_tool::u128_to_vec_u8;


pub struct InstanceDaoImpl;

impl InstanceDaoTrait for InstanceDaoImpl {
    fn insert(instance: &Instance) -> Result<usize> {
        use self::schema::instances;
        let new = RawInstance::new(instance)?;
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
            .filter(id.eq(ins.id.to_ne_bytes().to_vec()))
            .filter(thing.eq(ins.thing.key.clone()))
            .filter(version.eq(ins.thing.version))
            .filter(status_version.eq(ins.status_version))
            .order(status_version.desc())
            .limit(1)
            .load::<RawInstance>(conn);
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
            .load::<RawInstance>(conn)?;
        match def.len() {
            0 => Ok(None),
            1 => Ok(Some(def[0].to()?)),
            _ => Err(NatureErrorWrapper::from(NatureError::SystemError("should less than 2 record return".to_string()))),
        }
    }
}

impl InstanceDaoImpl {
    pub fn delete(ins: &Instance) -> Result<usize> {
        debug!("delete instance, id is : {:?}", ins.id);
        use self::schema::instances::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let rows = instances
            .filter(id.eq(ins.id.to_ne_bytes().to_vec()))
            .filter(thing.eq(ins.thing.key.clone()))
            .filter(version.eq(ins.thing.version))
            .filter(status_version.eq(ins.status_version));
        //        debug!("rows filter : {:?}", rows);
        let rtn = diesel::delete(rows).execute(conn)?;
        Ok(rtn)
    }
}


#[cfg(test)]
mod test {
    use data::sqlite::dao::instance::InstanceDaoImpl;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn instance_insert_exists_delete_test() {
        // prepare data to insert
        let instance = Instance {
            id: 0,
            data: InstanceNoID {
                thing: Thing {
                    key: "/instance/common".to_string(),
                    version: 100,
                    thing_type: ThingType::Business,
                },
                event_time: 0,
                execute_time: 0,
                create_time: 0,
                content: String::new(),
                context: HashMap::new(),
                status: HashSet::new(),
                status_version: 123,
                from: None,
            },
        };
        // delete if it exists
        if let Ok(true) = InstanceDaoImpl::is_exists(&instance) {
            let _ = InstanceDaoImpl::delete(&instance);
        }
        // insert one
        assert_eq!(Ok(1), InstanceDaoImpl::insert(&instance));
        // insert twice
        assert_eq!(InstanceDaoImpl::insert(&instance), Err(NatureErrorWrapper { err: NatureError::DaoDuplicated }));
        // exists
        assert_eq!(true, InstanceDaoImpl::is_exists(&instance).unwrap());
        // delete it
        assert_eq!(1, InstanceDaoImpl::delete(&instance).unwrap());
    }

    #[test]
    fn get_last_status() {
        // prepare data to insert
        let mut instance = Instance {
            id: 0,
            data: InstanceNoID {
                thing: Thing {
                    key: "/instance/getLast".to_string(),
                    version: 100,
                    thing_type: ThingType::Business,
                },
                event_time: 0,
                execute_time: 0,
                create_time: 0,
                content: String::new(),
                context: HashMap::new(),
                status: HashSet::new(),
                status_version: 123,
                from: None,
            },
        };
        // delete old if exists
        if let Ok(true) = InstanceDaoImpl::is_exists(&instance) {
            let _ = InstanceDaoImpl::delete(&instance);
        }
        instance.data.status_version = 111;
        if let Ok(true) = InstanceDaoImpl::is_exists(&instance) {
            let _ = InstanceDaoImpl::delete(&instance);
        }
        // insert one
        instance.data.status_version = 123;
        assert_eq!(Ok(1), InstanceDaoImpl::insert(&instance));
        // insert two
        instance.data.status_version = 111;
        assert_eq!(Ok(1), InstanceDaoImpl::insert(&instance));
        // get last
        if let Ok(Some(x)) = InstanceDaoImpl::get_by_id(instance.id) {
            assert_eq!(123, x.status_version);
        } else {
            panic!("shouldn't get error");
        }
        // delete after test
        let _ = InstanceDaoImpl::delete(&instance);
        instance.data.status_version = 123;
        let _ = InstanceDaoImpl::delete(&instance);
    }
}