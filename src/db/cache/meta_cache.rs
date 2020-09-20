use std::convert::TryInto;
use std::sync::Mutex;
use std::time::Duration;

use lru_time_cache::LruCache;

use crate::common::{Meta, MetaType, NatureError, Result};
use crate::db::MetaDao;

lazy_static! {
    pub static ref C_M: MetaCacheImpl = MetaCacheImpl {};
    static ref CACHE: Mutex<LruCache<String, Meta>> = Mutex::new(LruCache::<String, Meta>::with_expiry_duration(Duration::from_secs(3600)));
}

#[async_trait]
pub trait MetaCache: Sync + Send {
    async fn get<M>(&self, meta_str: &str, getter: &M) -> Result<Meta> where M: MetaDao;
}

#[derive(Copy, Clone)]
pub struct MetaCacheImpl;

#[async_trait]
impl MetaCache for MetaCacheImpl {
    async fn get<M>(&self, meta_str: &str, getter: &M) -> Result<Meta>
        where M: MetaDao
    {
        if meta_str.is_empty() {
            let error = NatureError::VerifyError("[biz] can not be empty!".to_string());
            warn!("{}", error);
            return Err(error);
        }
        // load from cache
        {   // An explicit scope to avoid cache.insert error
            let mut cache = CACHE.lock().unwrap();
            if let Some(x) = cache.get(meta_str) {
                return Ok(x.clone());
            };
        };

        // load from db
        let mut got: Vec<(String, Meta)> = vec![];
        let mut input: Vec<(String, ProcessType)> = vec![];
        input.push((meta_str.to_string(), ProcessType::Any));
        loop {
            let para = input.pop();
            if para.is_none() {
                break;
            }
            let para = para.unwrap();
            let meta = if let Some(def) = getter.get(&para.0).await? {
                let meta: Meta = def.try_into()?;
                if para.1 == ProcessType::NotState && meta.is_state() {
                    let msg = format!("{} could not be state", &para.0);
                    return Err(NatureError::VerifyError(msg));
                }
                match meta.get_meta_type() {
                    MetaType::Multi => {
                        if meta.is_state() {
                            let msg = format!("{} must not be state", &para.0);
                            return Err(NatureError::VerifyError(msg));
                        }
                        let sub = get_sub(&meta)?;
                        sub.into_iter().for_each(|one| input.push((one, ProcessType::NotState)));
                    }
                    MetaType::Loop => {
                        if meta.is_state() {
                            let msg = format!("{} must not be state", &para.0);
                            return Err(NatureError::VerifyError(msg));
                        }
                        let sub = get_sub(&meta)?;
                        // only_one required one item
                        if let Some(setting) = meta.get_setting() {
                            if setting.only_one && sub.len() != 1 {
                                let msg = format!("{}: only_one required only one item in sub but get {}", &para.0, sub.len());
                                return Err(NatureError::VerifyError(msg));
                            }
                        }
                        sub.into_iter().for_each(|one| input.push((one, ProcessType::NotState)));
                    }
                    _ => {
                        match get_master(&meta)? {
                            None => {}
                            Some(master) => input.push((master, ProcessType::Any)),
                        }
                    }
                }
                meta
            } else {
                get_none(&para.0)?
            };
            got.push((para.0, meta.clone()));
        }
        if got.len() > 0 {
            let mut cache = CACHE.lock().unwrap();
            got.iter().for_each(|one| {
                cache.insert(one.0.to_string(), one.1.clone());
            });
            Ok(got[0].1.clone())
        } else {
            get_none(meta_str)
        }
    }
}

#[derive(Eq, PartialEq)]
enum ProcessType {
    Any,
    NotState,
}

fn get_none(meta_str: &str) -> Result<Meta> {
    let m = Meta::from_string(meta_str)?;
    match m.get_meta_type() {
        MetaType::Null => {
            let mut cache = CACHE.lock().unwrap();
            cache.insert(meta_str.to_string(), m.clone());
            Ok(m)
        }
        MetaType::Dynamic => {
            let mut cache = CACHE.lock().unwrap();
            cache.insert(meta_str.to_string(), m.clone());
            Ok(m)
        }
        _ => {
            let error = NatureError::VerifyError(format!("{} not defined", meta_str));
            warn!("{}", error);
            Err(error)
        }
    }
}

fn get_sub(m: &Meta) -> Result<Vec<String>> {
    match m.get_setting() {
        None => Err(NatureError::VerifyError("Multi-Meta must define sub-metas".to_string())),
        Some(setting) => {
            let vec: Vec<String> = setting.multi_meta.into_iter().collect();
            let err = vec.iter().any(|one| {
                MetaType::check_type(one, MetaType::Loop).is_ok()
                    || MetaType::check_type(one, MetaType::Multi).is_ok()
            });
            if err {
                return Err(NatureError::VerifyError("MetaType: Multi or Loop can't be as sub meta".to_string()));
            }
            match vec.len() {
                0 => Err(NatureError::VerifyError("sub-meta number should great than 0".to_string())),
                _n => Ok(vec)
            }
        }
    }
}

fn get_master(meta: &Meta) -> Result<Option<String>> {
    match meta.get_setting() {
        None => Ok(None),
        Some(setting) => match setting.master {
            None => Ok(None),
            Some(master) => {
                Ok(Some(master))
            }
        },
    }
}

#[cfg(test)]
mod test {
    use std::collections::btree_set::BTreeSet;

    use crate::common::MetaSetting;
    use crate::db::RawMeta;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn cache_mater_test() {
        {   // clear cache
            let mut c = CACHE.lock().unwrap();
            c.clear();
        }
        let _rtn = C_M.get("B:child:1", &MetaMock {}).await.unwrap();
        {
            let mut c = CACHE.lock().unwrap();
            assert_eq!(3, c.len());
            let x = c.get("B:child:1").unwrap();
            assert_eq!(x.meta_string(), "B:child:1");
            let x = c.get("B:master:1").unwrap();
            assert_eq!(x.meta_string(), "B:master:1");
            let x = c.get("B:master-master:1").unwrap();
            assert_eq!(x.meta_string(), "B:master-master:1");
        }
    }

    #[tokio::test]
    #[ignore]
    async fn loop_self_must_not_state() {
        {   // clear cache
            let mut c = CACHE.lock().unwrap();
            c.clear();
        }
        let rtn = C_M.get("L:state:1", &MetaMock {}).await.err().unwrap();
        assert_eq!(NatureError::VerifyError("L:state:1 must not be state".to_string()), rtn);
        {
            let c = CACHE.lock().unwrap();
            assert_eq!(0, c.len());
        }
    }

    #[tokio::test]
    #[ignore]
    async fn loop_hss_none_sub() {
        {   // clear cache
            let mut c = CACHE.lock().unwrap();
            c.clear();
        }
        let rtn = C_M.get("L:none_sub:1", &MetaMock {}).await.err().unwrap();
        assert_eq!(NatureError::VerifyError("sub-meta number should great than 0".to_string()), rtn);
        {
            let c = CACHE.lock().unwrap();
            assert_eq!(0, c.len());
        }
    }

    #[tokio::test]
    #[ignore]
    async fn loop_has_multi_sub_include_state() {
        {   // clear cache
            let mut c = CACHE.lock().unwrap();
            c.clear();
        }
        let rtn = C_M.get("L:multi-include-state:1", &MetaMock {}).await.err().unwrap();
        assert_eq!(NatureError::VerifyError("B:sub-state:1 could not be state".to_string()), rtn);
        {
            let c = CACHE.lock().unwrap();
            assert_eq!(0, c.len());
        }
    }

    #[tokio::test]
    #[ignore]
    async fn loop_has_multi_sub_no_state() {
        {   // clear cache
            let mut c = CACHE.lock().unwrap();
            c.clear();
        }
        let _rtn = C_M.get("L:multi:1", &MetaMock {}).await.unwrap();
        {
            let c = CACHE.lock().unwrap();
            assert_eq!(3, c.len());
        }
    }

    #[tokio::test]
    #[ignore]
    async fn loop_has_one_sub_but_state() {
        let rtn = C_M.get("L:one-state:1", &MetaMock {}).await.err().unwrap();
        assert_eq!(NatureError::VerifyError("B:sub-state:1 could not be state".to_string()), rtn);
        {
            let c = CACHE.lock().unwrap();
            assert_eq!(0, c.len());
        }
    }

    #[tokio::test]
    #[ignore]
    async fn multi_must_not_state() {
        {   // clear cache
            let mut c = CACHE.lock().unwrap();
            c.clear();
        }
        let rtn = C_M.get("M:state:1", &MetaMock {}).await.err().unwrap();
        assert_eq!(NatureError::VerifyError("M:state:1 must not be state".to_string()), rtn);
        {
            let c = CACHE.lock().unwrap();
            assert_eq!(0, c.len());
        }
    }

    #[tokio::test]
    #[ignore]
    async fn multi_hss_none_sub() {
        {   // clear cache
            let mut c = CACHE.lock().unwrap();
            c.clear();
        }
        let rtn = C_M.get("M:none_sub:1", &MetaMock {}).await.err().unwrap();
        assert_eq!(NatureError::VerifyError("sub-meta number should great than 0".to_string()), rtn);
        {
            let c = CACHE.lock().unwrap();
            assert_eq!(0, c.len());
        }
    }

    #[tokio::test]
    #[ignore]
    async fn multi_one_state() {
        {   // clear cache
            let mut c = CACHE.lock().unwrap();
            c.clear();
        }
        let rtn = C_M.get("M:one-state:1", &MetaMock {}).await.err().unwrap();
        assert_eq!(NatureError::VerifyError("B:sub-state:1 could not be state".to_string()), rtn);
        {
            let c = CACHE.lock().unwrap();
            assert_eq!(0, c.len());
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sub_is_multi_or_loop() {
        {   // clear cache
            let mut c = CACHE.lock().unwrap();
            c.clear();
        }
        let rtn = C_M.get("M:sub:1", &MetaMock {}).await.err().unwrap();
        assert_eq!(NatureError::VerifyError("MetaType: Multi or Loop can't be as sub meta".to_string()), rtn);
        {
            let c = CACHE.lock().unwrap();
            assert_eq!(0, c.len());
        }
    }

    #[test]
    fn get_master_test() {
        let mut setting = MetaSetting {
            is_state: false,
            master: Some("abc".to_string()),
            multi_meta: Default::default(),
            cache_saved: false,
            only_one: false,
        };
        let mut m = Meta::from_string("B:test:3").unwrap();
        let _ = m.set_setting(&setting.to_json().unwrap());
        let rtn = get_master(&m).unwrap().unwrap();
        assert_eq!("abc", rtn);

        // for none
        setting.master = None;
        let _ = m.set_setting(&setting.to_json().unwrap());
        let rtn = get_master(&m).unwrap();
        assert_eq!(None, rtn);
    }

    #[test]
    fn get_sub_test() {
        let mut set: BTreeSet<String> = BTreeSet::new();
        set.insert("B:p/a:1".to_owned());
        set.insert("B:p/b:1".to_owned());
        let setting = MetaSetting {
            is_state: false,
            master: None,
            multi_meta: set,
            cache_saved: false,
            only_one: false,
        };
        let mut m = Meta::from_string("B:test:3").unwrap();
        let _ = m.set_setting(&setting.to_json().unwrap());
        let rtn = get_sub(&m).unwrap();
        assert_eq!(2, rtn.len());
        assert_eq!(true, rtn.contains(&"B:p/a:1".to_string()));
        assert_eq!(true, rtn.contains(&"B:p/b:1".to_string()));
    }

    #[derive(Copy, Clone)]
    struct MetaMock;

    #[async_trait]
    impl MetaDao for MetaMock {
        async fn get(&self, m: &str) -> Result<Option<RawMeta>> {
            let rtn = match m {
                "L:state:1" => {
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "state".to_string();
                    rtn.meta_type = "L".to_string();
                    rtn.config = r#"{"is_state":true}"#.to_string();
                    rtn
                }
                "L:none_sub:1" => {
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "none_sub".to_string();
                    rtn.meta_type = "L".to_string();
                    rtn
                }
                "L:multi-include-state:1" => {
                    let mut set: BTreeSet<String> = BTreeSet::new();
                    set.insert("B:sub-1:1".to_owned());
                    set.insert("B:sub-state:1".to_owned());
                    let setting = MetaSetting {
                        is_state: false,
                        master: None,
                        multi_meta: set,
                        cache_saved: false,
                        only_one: false,
                    };
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "sub-has-state".to_string();
                    rtn.meta_type = "L".to_string();
                    rtn.config = serde_json::to_string(&setting).unwrap();
                    rtn
                }
                "L:one-state:1" => {
                    let mut set: BTreeSet<String> = BTreeSet::new();
                    set.insert("B:sub-state:1".to_owned());
                    let setting = MetaSetting {
                        is_state: false,
                        master: None,
                        multi_meta: set,
                        cache_saved: false,
                        only_one: false,
                    };
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "one-state".to_string();
                    rtn.meta_type = "L".to_string();
                    rtn.config = serde_json::to_string(&setting).unwrap();
                    rtn
                }
                "L:multi:1" => {
                    let mut set: BTreeSet<String> = BTreeSet::new();
                    set.insert("B:sub-1:1".to_owned());
                    set.insert("B:sub-end:1".to_owned());
                    let setting = MetaSetting {
                        is_state: false,
                        master: None,
                        multi_meta: set,
                        cache_saved: false,
                        only_one: false,
                    };
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "multi".to_string();
                    rtn.meta_type = "L".to_string();
                    rtn.config = serde_json::to_string(&setting).unwrap();
                    rtn
                }
                "M:state:1" => {
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "state".to_string();
                    rtn.meta_type = "M".to_string();
                    rtn.config = r#"{"is_state":true}"#.to_string();
                    rtn
                }
                "M:none_sub:1" => {
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "none_sub".to_string();
                    rtn.meta_type = "M".to_string();
                    rtn
                }
                "M:one-state:1" => {
                    let mut set: BTreeSet<String> = BTreeSet::new();
                    set.insert("B:sub-state:1".to_owned());
                    let setting = MetaSetting {
                        is_state: false,
                        master: None,
                        multi_meta: set,
                        cache_saved: false,
                        only_one: false,
                    };
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "one-state".to_string();
                    rtn.meta_type = "M".to_string();
                    rtn.config = serde_json::to_string(&setting).unwrap();
                    rtn
                }
                "M:sub:1" => {
                    let mut set: BTreeSet<String> = BTreeSet::new();
                    set.insert("B:sub-1:1".to_owned());
                    set.insert("M:sub-2:1".to_owned());
                    let setting = MetaSetting {
                        is_state: false,
                        master: None,
                        multi_meta: set,
                        cache_saved: false,
                        only_one: false,
                    };
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "sub".to_string();
                    rtn.meta_type = "M".to_string();
                    rtn.config = serde_json::to_string(&setting).unwrap();
                    rtn
                }
                "B:sub-1:1" => {
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "sub-1".to_string();
                    rtn
                }
                "M:sub-2:1" => {
                    let mut set: BTreeSet<String> = BTreeSet::new();
                    set.insert("B:sub-end:1".to_owned());
                    let setting = MetaSetting {
                        is_state: false,
                        master: None,
                        multi_meta: set,
                        cache_saved: false,
                        only_one: false,
                    };
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "sub-2".to_string();
                    rtn.meta_type = "M".to_string();
                    rtn.config = serde_json::to_string(&setting).unwrap();
                    rtn
                }
                "B:sub-end:1" => {
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "sub-end".to_string();
                    rtn
                }
                "B:sub-state:1" => {
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "sub-state".to_string();
                    rtn.config = r#"{"is_state":true}"#.to_string();
                    rtn
                }
                "B:child:1" => {
                    let setting = MetaSetting {
                        is_state: false,
                        master: Some("B:master:1".to_string()),
                        multi_meta: Default::default(),
                        cache_saved: false,
                        only_one: false,
                    };
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "child".to_string();
                    rtn.config = serde_json::to_string(&setting).unwrap();
                    rtn
                }
                "B:master:1" => {
                    let setting = MetaSetting {
                        is_state: false,
                        master: Some("B:master-master:1".to_string()),
                        multi_meta: Default::default(),
                        cache_saved: false,
                        only_one: false,
                    };
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "master".to_string();
                    rtn.config = serde_json::to_string(&setting).unwrap();
                    rtn
                }
                "B:master-master:1" => {
                    let mut rtn = RawMeta::default();
                    rtn.meta_key = "master-master".to_string();
                    rtn
                }
                _ => return Err(NatureError::LogicalError("undefined meta".to_string()))
            };
            Ok(Some(rtn))
        }

        async fn insert(&self, _define: &RawMeta) -> Result<u64> {
            unimplemented!()
        }

        async fn update_flag(&self, _meta_str: &str, _flag_f: i32) -> Result<u64> {
            unimplemented!()
        }

        async fn delete(&self, _m: &Meta) -> Result<u64> {
            unimplemented!()
        }
    }
}