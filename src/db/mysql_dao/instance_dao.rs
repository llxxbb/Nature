use std::str::FromStr;

use chrono::{Local, TimeZone};
use mysql_async::{params, Value};

use crate::common::*;
use crate::db::{Mission, QUERY_SIZE_LIMIT};
use crate::db::mysql_dao::MySql;
use crate::db::raw_models::RawInstance;

#[async_trait]
pub trait KeyRange: Sync + Send {
    async fn get_by_key_range(&self, f_para: &KeyCondition) -> Result<Vec<Instance>>;
}

pub struct InstanceDaoImpl;

impl InstanceDaoImpl {
    pub async fn insert(instance: &Instance) -> Result<u64> {
        let new = RawInstance::new(instance)?;
        let sql = r"INSERT INTO instances
            (ins_key, content, context, states, state_version, create_time, sys_context, from_key)
            VALUES(:ins_key, :content,:context,:states,:state_version,:create_time,:sys_context,:from_key)";
        let vec: Vec<(String, Value)> = new.into();
        let rtn: u64 = match MySql::idu(sql, vec).await {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };
        debug!("Saved instance : {}", instance.get_key());
        Ok(rtn)
    }

    //noinspection RsLiveness
    /// check whether source stored earlier
    pub async fn get_by_from(f_para: &IDAndFrom) -> Result<Option<Instance>> {
        let sql = r"SELECT ins_key, content, context, states, state_version, create_time, sys_context, from_key
            FROM instances
            where ins_key like :para_like and from_key = :from_key
            order by state_version desc
            limit 1";
        let p = params! {
            "para_like" => f_para.para_like().to_string(),
            "from_key" => f_para.from_key.to_string(),
        };

        let rtn = MySql::fetch(sql, p, RawInstance::from).await?;
        match rtn.len() {
            1 => Ok(Some(rtn[0].to()?)),
            0 => Ok(None),
            _ => Err(NatureError::LogicalError("should not return more than one rows".to_string()))
        }
    }

    //noinspection RsLiveness
    async fn get_last_state(f_para: &KeyCondition) -> Result<Option<Instance>> {
        let sql = r"SELECT ins_key, content, context, states, state_version, create_time, sys_context, from_key
            FROM instances
            where ins_key = :ins_key
            order by state_version desc
            limit 1";
        let p = params! {
            "ins_key" => f_para.get_key(),
        };
        let rtn = MySql::fetch(sql, p, RawInstance::from).await?;
        match rtn.len() {
            1 => Ok(Some(rtn[0].to()?)),
            0 => Ok(None),
            _ => Err(NatureError::LogicalError("should not return more than one rows".to_string()))
        }
    }

    pub async fn get_by_key(key: String, spliter: String) -> Result<Option<Instance>> {
        let temp: Vec<&str> = key.split(&spliter).collect();
        if temp.len() != 4 {
            return Err(NatureError::VerifyError("error key format for task".to_string()));
        }
        let para = KeyCondition {
            id: temp[1].to_string(),
            meta: temp[0].to_string(),
            key_gt: "".to_string(),
            key_ge: "".to_string(),
            key_lt: "".to_string(),
            key_le: "".to_string(),
            para: temp[2].to_string(),
            state_version: i32::from_str(temp[3])?,
            time_ge: None,
            time_lt: None,
            limit: 1,
        };
        Self::get_by_id(para).await
    }

    //noinspection RsLiveness
    pub async fn get_by_id(f_para: KeyCondition) -> Result<Option<Instance>> {
        let sql = r"SELECT ins_key, content, context, states, state_version, create_time, sys_context, from_key
            FROM instances
            where ins_key = :ins_key and state_version = :state_version
            order by state_version desc
            limit 1";
        let p = params! {
            "ins_key" => f_para.get_key().to_string(),
            "state_version" => f_para.state_version,
        };
        let rtn = MySql::fetch(sql, p, RawInstance::from).await?;
        match rtn.len() {
            1 => Ok(Some(rtn[0].to()?)),
            0 => Ok(None),
            _ => Err(NatureError::LogicalError("should not return more than one rows".to_string()))
        }
    }

    pub async fn delete(ins: &Instance) -> Result<u64> {
        let sql = r"DELETE FROM instances
            WHERE ins_key=:ins_key";
        let p = params! {
            "ins_key" => ins.get_key(),
        };
        let rtn = MySql::idu(sql, p).await?;
        debug!("instance deleted, id is : {:?}", ins.id);
        Ok(rtn)
    }

    /// get downstream instance through upstream instance
    pub async fn get_last_target(from: &Instance, mission: &mut Mission) -> Result<Option<Instance>> {
        // init for MetaType::loop --------------------
        if mission.to.get_meta_type() == MetaType::Loop
            && mission.to.meta_string() == from.meta {
            if let Some(setting) = mission.to.get_setting() {
                if setting.only_one {
                    debug!("make MetaType::Loop as last state for {}", from.meta);
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
            let id = get_para_and_key_from_para(&from.para, para_part)?.0;
            mission.sys_context.insert(CONTEXT_TARGET_INSTANCE_PARA.to_string(), id.to_string());
            id
        } else {
            "".to_string()
        };
        let id = match mission.sys_context.get(&*CONTEXT_TARGET_INSTANCE_ID) {
            // context have target id
            Some(state_id) => state_id.to_string(),
            None => {
                if mission.use_upstream_id || mission.to.check_master(&from.meta) {
                    let from_id = format!("{:x}", from.id);
                    mission.sys_context.insert(CONTEXT_TARGET_INSTANCE_ID.to_string(), from_id.to_string());
                    from_id
                } else {
                    "0".to_string()
                }
            }
        };
        let meta = mission.to.meta_string();
        debug!("get last state for meta {}", &meta);
        let qc = KeyCondition::new(&id, &meta, &para_id, 0);
        Self::get_last_state(&qc).await
    }
}

#[async_trait]
impl KeyRange for InstanceDaoImpl {
    /// ins_key > and between time range
    async fn get_by_key_range(&self, f_para: &KeyCondition) -> Result<Vec<Instance>> {
        let key_like = if f_para.meta.is_empty() {
            ""
        } else {
            " and ins_key like :meta"
        };
        let key_gt = if f_para.key_gt.eq("") { "" } else {
            " and ins_key > :key_gt"
        };
        let key_ge = if f_para.key_ge.eq("") { "" } else {
            " and ins_key >= :key_ge"
        };
        let key_lt = if f_para.key_lt.eq("") { "" } else {
            " and ins_key < :key_lt"
        };
        let key_le = if f_para.key_le.eq("") { "" } else {
            " and ins_key <= :key_le"
        };
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
        let sql = format!("SELECT ins_key, content, context, states, state_version, create_time, sys_context, from_key
            FROM instances
            where 1=1{}{}{}{}{}{}{}
            order by ins_key
            limit :limit", time_ge, time_lt, key_gt, key_ge, key_lt, key_le, key_like);

        let p = params! {
"meta" => f_para.meta.to_string() + "%",
"key_gt" => f_para.key_gt.to_string(),
"key_ge" => f_para.key_ge.to_string(),
"key_lt" => f_para.key_lt.to_string(),
"key_le" => f_para.key_le.to_string(),
"time_ge" => Local.timestamp_millis(time_ge_v).naive_local(),
"time_lt" => Local.timestamp_millis(time_lt_v).naive_local(),
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


#[cfg(test)]
mod test {
    use std::env;

    use tokio::runtime::Runtime;

    use super::*;

    #[test]
    #[allow(dead_code)]
    fn get_last_state_test() {
        env::set_var("DATABASE_URL", "mysql://root@localhost/nature");
        let para = KeyCondition::new("0", "B:score/trainee/all-subject:1", "002", 0);
        let result = Runtime::new().unwrap().block_on(InstanceDaoImpl::get_last_state(&para));
        let _ = dbg!(result);
    }

    #[test]
    #[allow(dead_code)]
    fn query_by_id() {
        env::set_var("DATABASE_URL", "mysql://root@localhost/nature");
        let para = KeyCondition {
            id: "3827f37003127855b32ea022daa04cd".to_string(),
            meta: "B:sale/order:1".to_string(),
            key_gt: "".to_string(),
            key_ge: "".to_string(),
            key_lt: "".to_string(),
            key_le: "".to_string(),
            para: "".to_string(),
            state_version: 0,
            time_ge: None,
            time_lt: None,
            limit: 1,
        };
        let result = Runtime::new().unwrap().block_on(InstanceDaoImpl::get_by_id(para));
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
        let ge = Local.timestamp_millis(ge_t);
        dbg!(ge);
        let para = KeyCondition {
            id: "".to_string(),
            meta: "B:sale/order:1".to_string(),
            key_gt: "".to_string(),
            key_ge: "".to_string(),
            key_lt: "".to_string(),
            key_le: "".to_string(),
            para: "".to_string(),
            state_version: 0,
            time_ge: None,
            time_lt: None,
            limit: 100,
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
        let para = KeyCondition {
            id: "".to_string(),
            meta: "".to_string(),
            key_gt: "B:sale/order:1|".to_string(),
            key_ge: "".to_string(),
            key_lt: "B:sale/order:2|".to_string(),
            key_le: "".to_string(),
            para: "".to_string(),
            state_version: 0,
            time_ge: Some(
                1596115430000,
            ),
            time_lt: Some(
                1596115431000,
            ),
            limit: 20,
        };
        let dao = InstanceDaoImpl {};

        let result = dao.get_by_key_range(&para).await;
        assert!(result.is_ok());
        let vec = result.unwrap();
        dbg!(&vec);
    }
}