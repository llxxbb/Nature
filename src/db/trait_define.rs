use serde::Serialize;
use super::*;
use task::CarryError;

pub trait ThingDefineDao {
    fn get(thing: &Thing) -> Result<ThingDefine>;
    fn insert(define: &ThingDefine) -> Result<()>;
}

pub trait ThingDefineCacheTrait {
    fn get(thing: &Thing) -> Result<ThingDefine>;
}


pub trait MappingDao {
    fn get_relations(from: &Thing) -> Result<Vec<Mapping>>;
}

pub trait DeliveryDao {
    fn insert<T: Sized + Serialize>(carrier: &Carrier<T>) -> Result<u128>;
    fn delete(id: &u128) -> Result<()>;
    fn move_to_error<T: Sized + Serialize>(err: CarryError<T>) -> Result<()>;
    fn update_execute_time(_id: u128, new_time: i64) -> Result<()>;
    fn get<T: Sized + Serialize>(_id: u128) -> Result<Carrier<T>>;
}

pub trait InstanceDao {
    fn insert(instance: &Instance) -> Result<usize>;
    fn get_last_status_by_id(id: &u128) -> Result<Option<Instance>>;
    /// check whether source stored earlier
    fn is_exists(instance: &Instance) -> Result<bool>;
}

pub trait StorePlanDao {
    /// replace the plan if plan exists.
    fn save(plan: &StorePlan) -> Result<StorePlan>;
    fn get(from_id: &u128) -> Result<StorePlan>;
}
