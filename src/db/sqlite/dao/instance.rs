use diesel::prelude::*;
use std::ops::Deref;
use super::*;

pub struct TableInstance;

impl InstanceDao for TableInstance {
    fn insert(instance: &Instance) -> Result<()> {
        use self::schema::instances;
        let conn = DBPool::get_connection()?;
        let new = NewInstance::new(instance)?;
        let _ = diesel::insert_into(instances::table)
            .values(new)
            .execute(conn.deref());
        Ok(())
    }

    fn get_last_status_by_id(instance_id: &UuidBytes) -> Result<Option<Instance>> {
        use self::schema::instances::dsl::*;
        let conn = DBPool::get_connection()?;
        let def = instances.filter(id.eq(instance_id.to_vec()))
            .order(status_version.desc())
            .limit(1)
            .load::<NewInstance>(conn.deref())?;
        match def.len() {
            0 => Ok(None),
            1 => Ok(Some(Instance::from(def[0].clone())?)),
            _ => Err(NatureError::SystemError("should less than 2 record return".to_string())),
        }
    }
    /// check whether source stored earlier
    fn source_stored(_instance: &Instance) -> Result<bool> {
        unimplemented!()
    }
}