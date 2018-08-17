use global::*;
use serde::Serialize;
use std::fmt::Debug;
use super::*;

pub trait ThingDefineDaoTrait {
    fn get(thing: &Thing) -> Result<Option<ThingDefine>>;
    fn insert(define: &ThingDefine) -> Result<usize>;
    fn delete(thing: &Thing) -> Result<usize>;
}

pub trait ThingDefineCacheTrait {
    fn get(thing: &Thing) -> Result<ThingDefine>;
}


pub trait OneStepFlowDaoTrait {
    fn get_relations(from: &Thing) -> Result<Option<Vec<OneStepFlow>>>;
}

pub trait DeliveryDaoTrait {
    fn insert<T: Sized + Serialize + Send + Debug>(carrier: &Carrier<T>) -> Result<u128>;
    fn delete(id: u128) -> Result<()>;
    fn move_to_error<T: Sized + Serialize + Debug>(err: CarryError<T>) -> Result<()>;
    fn update_execute_time(_id: u128, new_time: i64) -> Result<()>;
    fn get<T: Sized + Serialize + Debug>(id: u128) -> Result<Carrier<T>>;
}

pub trait InstanceDaoTrait {
    fn insert(instance: &Instance) -> Result<usize>;
    /// check whether source stored earlier
    fn is_exists(instance: &Instance) -> Result<bool>;
    fn get_by_id(id: u128) -> Result<Option<Instance>>;
}

pub trait StorePlanDaoTrait {
    /// replace the plan if plan exists.
    fn save(plan: &PlanInfo) -> Result<PlanInfo>;
    fn get(from_id: &u128) -> Result<PlanInfo>;
}
