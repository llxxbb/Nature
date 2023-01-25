use std::collections::HashSet;
use std::sync::Arc;

use chrono::{Local, TimeZone};
use mysql_async::{params, Params, Row};
use crate::common::*;

use crate::db::Mission;
use crate::db::mysql_dao::MySql;
use crate::db::raw_models::RawInstance;
use crate::domain::*;
use crate::util::*;

#[async_trait]
pub trait KeyRange: Sync + Send {
    async fn get_by_key_range(&self, f_para: &InsCond) -> Result<Vec<Instance>>;
}

lazy_static! {
    pub static ref INS_RANGE : Arc<dyn KeyRange> = Arc::new(InstanceDaoImpl{});
}

pub struct InstanceDaoImpl;

impl InstanceDaoImpl {
    pub async fn insert(instance: &Instance) -> Result<u64> {
        let new = RawInstance::new(instance)?;
        let sql = r"INSERT INTO instances
            (meta, ins_id, para, content, context, states, state_version, create_time, sys_context, from_key)
            VALUES(:meta,:ins_id,:para,:content,:context,:states,:state_version,:create_time,:sys_context,:from_key)";
        let vec: Params = new.into();
        let rtn: u64 = match MySql::idu(sql, vec).await {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };
        debug!("Saved instance : {}", instance.get_key());
        Ok(rtn)
    }

    /// check whether source stored earlier
    pub async fn select_by_from(f_para: &IDAndFrom) -> Result<Option<Instance>> {
        let sql = r"SELECT meta, ins_id, para, content, context, states, state_version, create_time, sys_context, from_key
            FROM instances
            where meta = :meta and ins_id = :ins_id and from_key = :from_key
            order by state_version desc
            limit 1";
        let p = params! {
            "meta" => f_para.meta.to_string(),
            "ins_id" => f_para.id,
            "from_key" => f_para.from_key.to_string(),
        };

        let rtn = MySql::fetch(sql, p, RawInstance::from).await?;
        match rtn.len() {
            1 => Ok(Some(rtn[0].to()?)),
            0 => Ok(None),
            _ => Err(NatureError::LogicalError("should not return more than one rows".to_string()))
        }
    }

    /// get all downstream by `from`
    pub async fn select_downstream(from: &str) -> Result<Vec<Instance>> {
        let sql = r"SELECT meta, ins_id, para, content, context, states, state_version, create_time, sys_context, from_key
            FROM instances
            where from_key = :from_key";
        let p = params! {
            "from_key" => from.to_string(),
        };

        let raws = MySql::fetch(sql, p, RawInstance::from).await?;
        let mut rtn: Vec<Instance> = vec![];
        for one in raws {
            rtn.push(one.to()?)
        }
        Ok(rtn)
    }

    async fn select_last_state(f_para: &InsCond) -> Result<Option<Instance>> {
        let sql = r"SELECT meta, ins_id, para, content, context, states, state_version, create_time, sys_context, from_key
            FROM instances
            where meta = :meta and ins_id = :ins_id and para = :para
            order by state_version desc
            limit 1";
        let p = params! {
            "meta" => f_para.meta.to_string(),
            "ins_id" => f_para.id,
            "para" => f_para.para.to_string(),
        };
        let rtn = MySql::fetch(sql, p, RawInstance::from).await?;
        match rtn.len() {
            1 => Ok(Some(rtn[0].to()?)),
            0 => Ok(None),
            _ => Err(NatureError::LogicalError("should not return more than one rows".to_string()))
        }
    }

    pub async fn select_by_id(f_para: InsCond) -> Result<Option<Instance>> {
        let sql = r"SELECT meta, ins_id, para, content, context, states, state_version, create_time, sys_context, from_key
            FROM instances
            where meta = :meta and ins_id = :ins_id and para = :para and state_version = :state_version
            order by state_version desc
            limit 1";
        let p = params! {
            "meta" => f_para.meta.to_string(),
            "ins_id" => f_para.id,
            "para" => f_para.para.to_string(),
            "state_version" => f_para.state_version,
        };
        let rtn = MySql::fetch(sql, p, RawInstance::from).await?;
        match rtn.len() {
            1 => Ok(Some(rtn[0].to()?)),
            0 => Ok(None),
            _ => Err(NatureError::LogicalError("should not return more than one rows".to_string()))
        }
    }

    pub async fn delete(ins: &InstanceLocator) -> Result<u64> {
        let sql = r"DELETE FROM instances
            WHERE meta = :meta and ins_id = :ins_id and para = :para";
        let p = params! {
            "meta" => ins.meta.to_string(),
            "ins_id" => ins.id,
            "para" => ins.para.to_string(),
        };
        let rtn = MySql::idu(sql, p).await?;
        debug!("instance deleted, meta:id is : {}:{:?}", ins.meta, ins.id);
        Ok(rtn)
    }

    /// get downstream instance through upstream instance
    pub async fn select_last_target(from: &Instance, mission: &mut Mission) -> Result<Option<Instance>> {
        // init for MetaType::loop --------------------
        if mission.to.get_meta_type() == MetaType::Loop
            && mission.to.meta_string() == from.path.meta {
            if let Some(setting) = mission.to.get_setting() {
                if setting.only_one {
                    debug!("make MetaType::Loop as last state for {}", from.path.meta);
                    return Ok(Some(from.clone()));
                }
            }
        }
        // normal ---------------------------
        if !mission.to.is_state() {
            return Ok(None);
        }
        let para_part = &mission.target_demand.append_para;
        let para_id = if para_part.len() > 0 {
            let id = get_para_and_key_from_para(&from.path.para, para_part)?.0;
            mission.sys_context.insert(CONTEXT_TARGET_INSTANCE_PARA.to_string(), id.to_string());
            id
        } else {
            "".to_string()
        };
        let id = match mission.sys_context.get(&*CONTEXT_TARGET_INSTANCE_ID) {
            // context have target id
            Some(state_id) => state_id.to_string(),
            None => {
                if mission.use_upstream_id || mission.to.check_master(&from.path.meta) {
                    let from_id = format!("{}", from.id);
                    mission.sys_context.insert(CONTEXT_TARGET_INSTANCE_ID.to_string(), from_id.to_string());
                    from_id
                } else {
                    "0".to_string()
                }
            }
        };
        let meta = mission.to.meta_string();
        // debug!("get last state for meta {}", &meta);
        let qc = InsCond::new(id.parse()?, &meta, &para_id, 0);
        Self::select_last_state(&qc).await
    }

    pub async fn meta_exists(meta: &str) -> Result<bool> {
        let sql = "SELECT 1 FROM instances WHERE meta=:meta limit 1;";
        let p = params! {
            "meta" => meta.to_string(),
        };
        let rtn = MySql::fetch(sql, p, raw_to_number).await?;
        if rtn.len() > 0 { Ok(true) } else { Ok(false) }
    }
}

fn raw_to_number(row: Row) -> i32 {
    mysql_async::from_row(row)
}

#[async_trait]
impl KeyRange for InstanceDaoImpl {
    /// ins_key > and between time range
    async fn get_by_key_range(&self, f_para: &InsCond) -> Result<Vec<Instance>> {
        let mut list: Vec<String> = vec![];
        // used to avoid repeat add conditions
        let mut set: HashSet<String> = HashSet::new();

        let meta = if f_para.meta.is_empty() {
            ""
        } else {
            " and meta = :meta"
        };
        // key
        if !f_para.key_gt.eq("") {
            build_for_part(&mut set, &mut list, &f_para.key_gt, ">")?;
        };
        if !f_para.key_ge.eq("") {
            build_for_part(&mut set, &mut list, &f_para.key_ge, ">=")?;
        };
        if !f_para.key_lt.eq("") {
            build_for_part(&mut set, &mut list, &f_para.key_lt, "<")?;
        };
        if !f_para.key_le.eq("") {
            build_for_part(&mut set, &mut list, &f_para.key_le, "<=")?;
        };
        let key = list.join("");

        // other
        let time_ge = match f_para.time_ge {
            Some(_) => " and create_time >= :time_ge",
            None => ""
        };
        let time_ge_v = match f_para.time_ge {
            Some(ge) => ge,
            None => 0
        };
        let time_lt = match f_para.time_lt {
            Some(_) => " and create_time < :time_lt",
            None => ""
        };
        let time_lt_v = match f_para.time_lt {
            Some(lt) => lt,
            None => 0
        };
        let limit = if f_para.limit < *QUERY_SIZE_LIMIT {
            f_para.limit
        } else { *QUERY_SIZE_LIMIT };
        // sql
        let sql = format!("SELECT meta, ins_id, para, content, context, states, state_version, create_time, sys_context, from_key
            FROM instances
            where 1=1{}{}{}{}
            order by meta, create_time desc, state_version desc
            limit :limit", time_ge, time_lt, key, meta);

        let p = params! {
            "meta" => f_para.meta.to_string(),
            "time_ge" => Local.timestamp_millis_opt(time_ge_v).unwrap().naive_local(),
            "time_lt" => Local.timestamp_millis_opt(time_lt_v).unwrap().naive_local(),
            "limit" => limit,
        };
        dbg!(&sql);
        let result = MySql::fetch(sql, p, RawInstance::from).await?;
        let mut rtn: Vec<Instance> = vec![];
        for one in result {
            rtn.push(one.to()?)
        }
        Ok(rtn)
    }
}

fn key_to_part(key: &str) -> Vec<String> {
    if key.is_empty() {
        return vec![];
    }
    let parts: Vec<&str> = key.split(&*SEPARATOR_INS_KEY).collect();
    let mut rtn: Vec<String> = vec![];
    for part in parts {
        rtn.push(part.to_string());
    }
    rtn
}

/// generate where clause for query, ignore the parts more than 3
fn build_for_part(set: &mut HashSet<String>, list: &mut Vec<String>, parts: &str, end_sign: &str) -> Result<()> {
    if parts.contains("'") {
        return Err(NatureError::VerifyError("illegal query condition!".to_string()));
    }
    let vec = key_to_part(parts);
    if vec.len() > 1 {
        if set.insert(vec[0].clone()) {
            list.push(" and meta = '".to_owned() + &vec[0] + "'")
        }
    } else if vec.len() == 1 {
        list.push(" and meta ".to_owned() + end_sign + " '" + &vec[0] + "'")
    }
    if vec.len() > 2 {
        let id = if vec[1].is_empty() { "0" } else { &vec[1] };
        if set.insert(vec[0].clone() + &*SEPARATOR_INS_KEY + &vec[1]) {
            list.push(" and ins_id = ".to_owned() + id)
        }
    } else if vec.len() == 2 {
        let id = if vec[1].is_empty() { "0" } else { &vec[1] };
        list.push(" and ins_id ".to_owned() + end_sign + " " + id)
    }
    if vec.len() > 3 {
        if set.insert(vec[0].clone() + &*SEPARATOR_INS_KEY + &vec[1] + &*SEPARATOR_INS_KEY + &vec[2]) {
            list.push(" and para = '".to_owned() + &vec[2] + "'")
        }
    } else if vec.len() == 3 {
        list.push(" and para ".to_owned() + end_sign + " '" + &vec[2] + "'")
    }
    if vec.len() == 4 {
        list.push(" and state_version ".to_owned() + end_sign + " " + &vec[3])
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::env;

    use tokio::runtime::Runtime;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn insert_test() {
        env::set_var("DATABASE_URL", "mysql://root@localhost/nature");
        let instance = Instance::new("test").unwrap();
        let rtn = InstanceDaoImpl::insert(&instance).await.unwrap();
        assert_eq!(true, rtn > 0);
        let _ = dbg!(rtn);

        // meta_exists test---------------
        let rtn = InstanceDaoImpl::meta_exists("B:test:1").await.unwrap();
        assert_eq!(true, rtn);
        let rtn = InstanceDaoImpl::meta_exists("B:test:2").await.unwrap();
        assert_eq!(false, rtn);
    }

    #[test]
    #[ignore]
    #[allow(dead_code)]
    fn get_last_state_test() {
        env::set_var("DATABASE_URL", "mysql://root@localhost/nature");
        let para = InsCond::new(0, "B:score/trainee/all-subject:1", "002", 0);
        let result = Runtime::new().unwrap().block_on(InstanceDaoImpl::select_last_state(&para));
        let _ = dbg!(result);
    }

    #[test]
    #[ignore]
    #[allow(dead_code)]
    fn query_by_id() {
        env::set_var("DATABASE_URL", "mysql://root@localhost/nature");
        let para = InsCond {
            id: 12345,
            time_ge: None,
            time_lt: None,
            other: NoIdCond {
                meta: "B:sale/order:1".to_string(),
                key_gt: "".to_string(),
                key_ge: "".to_string(),
                key_lt: "".to_string(),
                key_le: "".to_string(),
                para: "".to_string(),
                state_version: 0,
                limit: 1,
            },
        };
        let result = Runtime::new().unwrap().block_on(InstanceDaoImpl::select_by_id(para));
        let _ = dbg!(result);
    }

    #[allow(dead_code)]
    #[tokio::test]
    #[ignore]
    async fn query_by_range_test() {
        env::set_var("DATABASE_URL", "mysql://root@localhost/nature");
        let mut ins = Instance::new("sale/order").unwrap();
        ins.id = 760228;
        let _ = InstanceDaoImpl::insert(&ins).await;

        let ge_t = 1588508143000;
        let ge = Local.timestamp_millis_opt(ge_t).unwrap();
        dbg!(ge);
        let para = InsCond {
            id: 0,
            time_ge: None,
            time_lt: None,
            other: NoIdCond {
                meta: "B:sale/order:1".to_string(),
                key_gt: "".to_string(),
                key_ge: "".to_string(),
                key_lt: "".to_string(),
                key_le: "".to_string(),
                para: "".to_string(),
                state_version: 0,
                limit: 100,
            },
        };
        let dao = InstanceDaoImpl {};

        let result = dao.get_by_key_range(&para).await;
        dbg!(&result);
        assert!(result.is_ok());
        let vec = result.unwrap();
        dbg!(vec);
    }

    #[allow(dead_code)]
    #[tokio::test]
    #[ignore]
    async fn query_by_range() {
        env::set_var("DATABASE_URL", "mysql://root@localhost/nature");
        let para = InsCond {
            id: 0,
            time_ge: Some(
                1596115430000,
            ),
            time_lt: Some(
                1596115431000,
            ),
            other: NoIdCond {
                meta: "".to_string(),
                key_gt: "B:sale/order:1|".to_string(),
                key_ge: "".to_string(),
                key_lt: "B:sale/order:2|".to_string(),
                key_le: "".to_string(),
                para: "".to_string(),
                state_version: 0,
                limit: 20,
            },
        };
        let dao = InstanceDaoImpl {};

        let result = dao.get_by_key_range(&para).await;
        assert!(result.is_ok());
        let vec = result.unwrap();
        dbg!(&vec);
    }

    #[allow(dead_code)]
    #[tokio::test]
    #[ignore]
    async fn get_downstream_test() {
        env::set_var("DATABASE_URL", "mysql://root@localhost/nature");

        let result = InstanceDaoImpl::select_downstream("B:finance/payment:1|1193619470850765623||0").await;
        assert!(result.is_ok());
        let vec = result.unwrap();
        dbg!(&vec);
    }

    #[test]
    fn key_to_part_test() {
        let vec = key_to_part("a||b");
        assert_eq!(3, vec.len());
        assert_eq!("a", vec[0]);
        assert_eq!("", vec[1]);
        assert_eq!("b", vec[2]);

        let vec = key_to_part("a|b|");
        assert_eq!(3, vec.len());
        assert_eq!("a", vec[0]);
        assert_eq!("b", vec[1]);
        assert_eq!("", vec[2]);

        let vec = key_to_part("a|b");
        assert_eq!(2, vec.len());
        assert_eq!("a", vec[0]);
        assert_eq!("b", vec[1]);
    }
}

#[cfg(test)]
mod build_for_part_test {
    use super::*;

    #[test]
    fn error_input_test() {
        let mut list: Vec<String> = vec![];
        let mut set: HashSet<String> = HashSet::new();
        let result = build_for_part(&mut set, &mut list, "dfafdf|fdfa'fdsa|dfsadfasu", "");
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn empty_input_test() {
        let mut list: Vec<String> = vec![];
        let mut set: HashSet<String> = HashSet::new();
        let result = build_for_part(&mut set, &mut list, "", "");
        assert_eq!(true, result.is_ok());
        assert_eq!(0, list.len());
        assert_eq!(0, set.len());
    }

    #[test]
    fn meta_test() {
        let mut list: Vec<String> = vec![];
        let mut set: HashSet<String> = HashSet::new();
        let _ = build_for_part(&mut set, &mut list, "a", ">");
        let _ = build_for_part(&mut set, &mut list, "b", "<");
        assert_eq!(0, set.len());
        assert_eq!(2, list.len());
        assert_eq!(" and meta > 'a'", list[0]);
        assert_eq!(" and meta < 'b'", list[1]);
    }

    #[test]
    fn id_test() {
        let mut list: Vec<String> = vec![];
        let mut set: HashSet<String> = HashSet::new();
        let _ = build_for_part(&mut set, &mut list, "m|1", ">");
        let _ = build_for_part(&mut set, &mut list, "m|5", "<");
        assert_eq!(1, set.len());
        assert_eq!(true, set.contains("m"));
        assert_eq!(3, list.len());
        assert_eq!(" and meta = 'm'", list[0]);
        assert_eq!(" and ins_id > 1", list[1]);
        assert_eq!(" and ins_id < 5", list[2]);
    }

    #[test]
    fn id_err_test() {
        let mut list: Vec<String> = vec![];
        let mut set: HashSet<String> = HashSet::new();
        let _ = build_for_part(&mut set, &mut list, "m|1", ">");
        let _ = build_for_part(&mut set, &mut list, "a|5", "<");
        assert_eq!(2, set.len());
        assert_eq!(true, set.contains("m"));
        assert_eq!(true, set.contains("a"));
        assert_eq!(4, list.len());
        assert_eq!(" and meta = 'm'", list[0]);
        assert_eq!(" and ins_id > 1", list[1]);
        assert_eq!(" and meta = 'a'", list[2]);
        assert_eq!(" and ins_id < 5", list[3]);
    }

    #[test]
    fn id_empty_end_sign_test() {
        let mut list: Vec<String> = vec![];
        let mut set: HashSet<String> = HashSet::new();
        let _ = build_for_part(&mut set, &mut list, "m|", ">");
        assert_eq!(1, set.len());
        assert_eq!(true, set.contains("m"));
        assert_eq!(2, list.len());
        assert_eq!(" and meta = 'm'", list[0]);
        assert_eq!(" and ins_id > 0", list[1]);
    }

    #[test]
    fn id_empty_equal_test() {
        let mut list: Vec<String> = vec![];
        let mut set: HashSet<String> = HashSet::new();
        let _ = build_for_part(&mut set, &mut list, "m||a", ">");
        assert_eq!(2, set.len());
        assert_eq!(true, set.contains("m"));
        assert_eq!(true, set.contains("m|"));
        assert_eq!(3, list.len());
        assert_eq!(" and meta = 'm'", list[0]);
        assert_eq!(" and ins_id = 0", list[1]);
        assert_eq!(" and para > 'a'", list[2]);
    }

    #[test]
    fn para_test() {
        let mut list: Vec<String> = vec![];
        let mut set: HashSet<String> = HashSet::new();
        let _ = build_for_part(&mut set, &mut list, "m|0|a", ">");
        let _ = build_for_part(&mut set, &mut list, "m|0|b", "<");
        assert_eq!(2, set.len());
        assert_eq!(true, set.contains("m"));
        assert_eq!(true, set.contains("m|0"));
        assert_eq!(4, list.len());
        assert_eq!(" and meta = 'm'", list[0]);
        assert_eq!(" and ins_id = 0", list[1]);
        assert_eq!(" and para > 'a'", list[2]);
        assert_eq!(" and para < 'b'", list[3]);
    }

    #[test]
    fn status_test() {
        let mut list: Vec<String> = vec![];
        let mut set: HashSet<String> = HashSet::new();
        let _ = build_for_part(&mut set, &mut list, "m|0|a|dfdfd", ">");
        let _ = build_for_part(&mut set, &mut list, "m|0|a|eeefddi", "<");
        assert_eq!(3, set.len());
        assert_eq!(true, set.contains("m"));
        assert_eq!(true, set.contains("m|0"));
        assert_eq!(true, set.contains("m|0|a"));
        assert_eq!(5, list.len());
        assert_eq!(" and meta = 'm'", list[0]);
        assert_eq!(" and ins_id = 0", list[1]);
        assert_eq!(" and para = 'a'", list[2]);
        assert_eq!(" and state_version > dfdfd", list[3]);
        assert_eq!(" and state_version < eeefddi", list[4]);
    }

    #[test]
    fn para_empty_status_test() {
        let mut list: Vec<String> = vec![];
        let mut set: HashSet<String> = HashSet::new();
        let _ = build_for_part(&mut set, &mut list, "m|0||dfdfd", ">");
        assert_eq!(3, set.len());
        assert_eq!(true, set.contains("m"));
        assert_eq!(true, set.contains("m|0"));
        assert_eq!(true, set.contains("m|0|"));
        assert_eq!(4, list.len());
        assert_eq!(" and meta = 'm'", list[0]);
        assert_eq!(" and ins_id = 0", list[1]);
        assert_eq!(" and para = ''", list[2]);
        assert_eq!(" and state_version > dfdfd", list[3]);
    }
}