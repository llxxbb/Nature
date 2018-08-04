use diesel::prelude::*;
use super::*;

pub struct OneStepFlowDaoImpl;

impl OneStepFlowDaoTrait for OneStepFlowDaoImpl {
    fn get_relations(from: &Thing) -> Result<Option<Vec<OneStepFlow>>> {
        use self::schema::one_step_flow::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let def = one_step_flow
            .filter(from_thing.eq(&from.key))
            .filter(from_version.eq(from.version))
            .load::<OneStepFlowRow>(conn)?;
        match def.len() {
            0 => Ok(None),
            x if x > 0 => {
                let mut rtn: Vec<OneStepFlow> = Vec::new();
                for d in def {
                    if let Ok(one) = OneStepFlow::from_row(d) {
                        rtn.push(one);
                    }
                }
                if rtn.len() > 0 {
                    return Ok(Some(rtn));
                } else {
                    return Ok(None);
                }
            }
            _ => Err(NatureError::SystemError("unknown error occurred".to_string())),
        }
    }
}

impl OneStepFlowDaoImpl {
    pub fn insert(one: OneStepFlowRow) {
        use self::schema::one_step_flow;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        diesel::insert_into(one_step_flow::table)
            .values(one)
            .execute(conn);
    }
}