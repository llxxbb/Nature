use data::PlanInfo;
use diesel::prelude::*;
use nature_common::*;
use super::*;

pub struct StorePlanDaoImpl;

impl StorePlanDaoTrait for StorePlanDaoImpl {
    fn save(plan: &PlanInfo) -> Result<()> {
        use self::schema::plan;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let will_save = RawPlanInfo::new(plan)?;
        let upstream = will_save.upstream.clone();
        let rtn = diesel::insert_into(plan::table)
            .values(will_save)
            .execute(conn);
        match rtn {
            Ok(x) => match x {
                1 => Ok(()),
                num => Err(NatureErrorWrapper::from(NatureError::DaoLogicalError(format!("should insert 1 but get {}", num)))),
            },
            Err(e) => Err({
                let wapper = NatureErrorWrapper::from(e);
                match wapper.err {
                    NatureError::DaoDuplicated(_) => NatureErrorWrapper::from(NatureError::DaoDuplicated(upstream)),
                    _ => wapper,
                }
            }),
        }
    }

    fn get(key: &str) -> Result<Option<PlanInfo>> {
        use super::schema::plan::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let def = plan.filter(upstream.eq(&key))
            .load::<RawPlanInfo>(conn)?;
        match def.len() {
            0 => Ok(None),
            1 => Ok(Some(def[0].to_plan_info()?)),
            x => Err(NatureErrorWrapper::from(NatureError::DaoLogicalError(format!("not 1 and 0 but get {}", x)))),
        }
    }
}
