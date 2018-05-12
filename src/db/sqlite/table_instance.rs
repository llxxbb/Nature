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
    fn get_last_status_by_id(_id: &UuidBytes) -> Result<Option<Instance>> {
        unimplemented!()
    }
    /// check whether source stored earlier
    fn source_stored(_instance: &Instance) -> Result<bool> {
        unimplemented!()
    }
}