use diesel::prelude::*;
use super::*;

pub struct OneStepFlowDaoImpl;

impl OneStepFlowDaoTrait for OneStepFlowDaoImpl {
    fn get_relations(from: &Thing) -> Result<Option<Vec<OneStepFlow>>> {
        use self::schema::one_step_flow::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let def = one_step_flow
            .filter(from_thing.eq(from.key))
            .filter(from_version.eq(from.version))
            .load::<OneStepFlowRow>(conn)?;
        // TODO
        match def.len() {
            0 => Ok(None),
            x if x > 0 => Ok(Some(Instance::from(def[0].clone())?)),
            _ => Err(NatureError::SystemError("unknown error occurred".to_string())),
        }
    }
}