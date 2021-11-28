use std::future::Future;

use mysql_async::{params, Value};

use crate::db::MySql;
use crate::db::raw_models::RawMeta;
use crate::domain::*;

lazy_static! {
    // Dao of Meta
    pub static ref D_M: MetaDaoImpl = MetaDaoImpl {};
}

pub type MetaGetter = fn(&str) -> dyn Future<Output=Result<Option<RawMeta>>>;

#[async_trait]
pub trait MetaDao: Sync + Send {
    async fn get(&self, meta_str: &str) -> Result<Option<RawMeta>>;
    async fn insert(&self, define: &RawMeta) -> Result<u64>;
    async fn update_flag(&self, meta_str: &str, flag_f: i32) -> Result<u64>;
    async fn edit(&self, define: &RawMeta) -> Result<u64>;
    async fn delete(&self, m: &Meta) -> Result<u64>;
    async fn id_great_than(&self, from: i32, limit: i32) -> Result<Vec<RawMeta>>;
}

pub struct MetaDaoImpl;

#[async_trait]
impl MetaDao for MetaDaoImpl {
    async fn get(&self, meta_str: &str) -> Result<Option<RawMeta>> {
        let sql = r"SELECT id, meta_type, meta_key, description, version, states, fields, config, flag, create_time
            FROM meta
            WHERE meta_type = :meta_type and meta_key = :meta_key and version = :version and flag = 1";

        let m = Meta::from_string(&meta_str)?;
        let p = params! {
            "meta_type" => m.get_meta_type().get_prefix(),
            "meta_key" => m.get_key(),
            "version" => m.version,
        };

        let rtn = MySql::fetch(sql, p, RawMeta::from).await?;
        match rtn.len() {
            1 => {
                let meta = rtn[0].clone();
                debug!("load meta : {:?}", &rtn);
                Ok(Some(meta))
            }
            0 => Ok(None),
            _ => Err(NatureError::LogicalError("should not return more than one rows".to_string()))
        }
    }
    async fn id_great_than(&self, from: i32, limit: i32) -> Result<Vec<RawMeta>> {
        let sql = r"SELECT id, meta_type, meta_key, description, version, states, fields, config, flag, create_time
            FROM meta
            WHERE id > :from
            order by id
            limit :limit";

        let p = params! {
            "from" => from,
            "limit" => limit,
        };
        let rtn = MySql::fetch(sql, p, RawMeta::from).await?;
        Ok(rtn)
    }

    async fn insert(&self, define: &RawMeta) -> Result<u64> {
        let sql = r"INSERT INTO meta
            (meta_type, meta_key, description, version, states, fields, config, flag, create_time)
            VALUES(:meta_type, :meta_key, :description, :version, :states, :fields, :config, :flag, :create_time)";
        let p: Vec<(String, Value)> = define.clone().into();
        let rtn = MySql::idu(sql, p).await?;
        debug!("Saved meta : {}:{}:{}", define.meta_type, define.meta_key, define.version);
        Ok(rtn)
    }

    async fn edit(&self, define: &RawMeta) -> Result<u64> {
        let sql = r"UPDATE meta SET
            description=:description,
            states=:states,
            fields=:fields,
            config=:config,
            flag=:flag,
            meta_type=:meta_type,
            meta_key=:meta_key
        WHERE id=:id AND version=:version;";
        let p: Vec<(String, Value)> = define.clone().into();
        let rtn = MySql::idu(sql, p).await?;
        debug!("updated meta : {}:{}:{}", define.meta_type, define.meta_key, define.version);
        Ok(rtn)
    }

    async fn update_flag(&self, meta_str: &str, flag_f: i32) -> Result<u64> {
        let sql = r"UPDATE meta
            SET flag=:flag
            WHERE meta_type = :meta_type and meta_key = :meta_key and version = :version";

        let m = Meta::from_string(meta_str)?;
        let p = params! {
            "meta_type" => m.get_meta_type().get_prefix(),
            "meta_key" => m.get_key(),
            "version" => m.version,
            "flag" => flag_f,
        };
        let rtn = MySql::idu(sql, p).await?;
        debug!("meta flag updated: {}:{}:{}", m.get_meta_type().get_prefix(), m.get_key(), m.version);
        Ok(rtn)
    }

    async fn delete(&self, m: &Meta) -> Result<u64> {
        let sql = r"DELETE FROM meta
            WHERE meta_type = :meta_type and meta_key = :meta_key and version = :version";

        let p = params! {
            "meta_type" => m.get_meta_type().get_prefix(),
            "meta_key" => m.get_key(),
            "version" => m.version,
        };

        let rtn = MySql::idu(sql, p).await?;
        debug!("meta deleted: {}:{}:{}", m.get_meta_type().get_prefix(), m.get_key(), m.version);
        Ok(rtn)
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use chrono::prelude::*;
    use tokio::runtime::Runtime;

    use crate::db::CONN_STR;

    use super::*;

    #[test]
    #[ignore]
    fn define_test() {
        // prepare data to insert
        env::set_var("DATABASE_URL", CONN_STR);
        let mut define = RawMeta {
            id: 0,
            meta_type: "B".to_string(),
            description: Some("description".to_string()),
            version: 100,
            states: Some("status".to_string()),
            fields: Some("fields".to_string()),
            config: "{}".to_string(),
            flag: 1,
            create_time: Local::now().naive_local(),
            meta_key: "test".to_string(),
        };
        let meta = "B:test:100";
        let m = Meta::from_string(meta).unwrap();
        // delete if it exists
        let mut runtime = Runtime::new().unwrap();
        if let Ok(Some(_)) = runtime.block_on(D_M.get("B:test:100")) {
            let _ = runtime.block_on(D_M.delete(&m));
        }

        // insert
        let rtn = runtime.block_on(D_M.insert(&define)).unwrap();
        assert_eq!(rtn > 0, true);
        define.id = rtn as i32;

        // repeat insert
        let rtn = runtime.block_on(D_M.insert(&define));
        let _ = match &rtn {
            Err(err) => match err {
                NatureError::DaoDuplicated(_) => (),
                _ => panic!("match error"),
            }
            _ => panic!("match error")
        };
        // update
        define.fields = Some("hello".to_string());
        let _ = runtime.block_on(D_M.edit(&define));

        // find inserted
        let mut row: RawMeta = runtime.block_on(D_M.get(meta)).unwrap().unwrap();
        row.create_time = define.create_time;
        assert_eq!(row, define);

        // change flag
        let _ = runtime.block_on(D_M.update_flag("B:test:100", 0));
        let row = runtime.block_on(D_M.get(meta)).unwrap();
        assert_eq!(row, None);

        // delete it
        let _ = runtime.block_on(D_M.delete(&m));
    }
}