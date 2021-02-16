use std::collections::{HashMap, HashSet};
use std::collections::btree_map::BTreeMap;
use std::str::FromStr;

use crate::domain::*;
use crate::util::*;

/// Business Metadata
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Meta {
    /// # Identify a `Meta`.
    ///
    /// A `Meta` may have a lots of `Instance`s, so it's a **Class** for Instance`.
    /// Because there are huge quantity of `Meta`s , so we need a way to organize `Meta`s.
    /// A way is to set name with hierarchical structures,
    key: String,
    /// A `Meta` can be changed in future, the `version` will support this without effect the old ones
    pub version: u32,
    /// A `Meta`'s type
    meta_type: MetaType,
    state: Option<States>,
    is_state: bool,
    setting: Option<MetaSetting>,
    /// hold all string-state, used to accelerate the check speed.
    check_list: BTreeMap<String, StatePath>,
    meta: String,
}


impl Default for Meta {
    fn default() -> Self {
        let full_key = MetaType::Business.get_prefix();
        Meta {
            key: String::new(),
            version: 1,
            meta_type: MetaType::Business,
            state: None,
            is_state: false,
            setting: None,
            check_list: Default::default(),
            meta: full_key + &*SEPARATOR_META + &1.to_string(),
        }
    }
}

impl Meta {
    /// make start with "/" and remove "/" at the end
    pub fn key_standardize(biz: &str) -> Result<String> {
        let mut biz = biz.to_string();
        if biz.ends_with(&*SEPARATOR_META_KEY) {
            let last = biz.len() - 1;
            biz.remove(last);
        }
        if biz.starts_with(&*SEPARATOR_META_KEY) {
            biz = biz[1..].to_string();
        }
        if biz.is_empty() {
            return Err(NatureError::VerifyError("key length can't be zero".to_string()));
        }
        if biz.contains(&*SEPARATOR_META) {
            return Err(NatureError::VerifyError("key can not contains [:] character".to_string()));
        }
        Ok(biz)
    }

    pub fn new(key: &str, version: u32, meta_type: MetaType) -> Result<Self> {
        let key = match meta_type {
            MetaType::Null => "".to_string(),
            _ => Self::key_standardize(key)?
        };
        let prefix = meta_type.get_prefix();
        Ok(Meta {
            key: key.to_string(),
            version,
            meta_type,
            state: None,
            is_state: false,
            setting: None,
            check_list: Default::default(),
            meta: prefix + &*SEPARATOR_META + &key + &*SEPARATOR_META + &version.to_string(),
        })
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_meta_type(&self) -> MetaType {
        self.meta_type.clone()
    }
    pub fn set_meta_type(&mut self, meta_type: MetaType) {
        self.meta_type = meta_type.clone();
        self.meta = meta_type.get_prefix() + &self.key + &*SEPARATOR_META + &1.to_string()
    }

    /// `meta_str`'s format : [MetaType]:[key]:[version]
    pub fn from_string(meta_str: &str) -> Result<Meta> {
        let x: Vec<&str> = meta_str.split(&*SEPARATOR_META).collect();
        if x.len() != 3 {
            return Err(NatureError::VerifyError("format should be [MetaType]:[key]:[version]".to_string()));
        }
        let meta_type = MetaType::from_prefix(x[0])?;
        if meta_type == MetaType::Null {
            return Meta::new("", 1, MetaType::Null);
        }
        let version = match x[2].parse::<u32>() {
            Ok(ver) => ver,
            Err(_) => return Err(NatureError::VerifyError("the end of the meta_str should be i32 type".to_string())),
        };
        Meta::new(x[1], version, meta_type)
    }

    pub fn has_state_name(&self, name: &str) -> bool {
        let option = self.check_list.get(name);
        option.is_some()
    }

    pub fn meta_string(&self) -> String {
        self.meta.clone()
    }

    pub fn set_states(&mut self, states: Option<States>) -> Result<()> {
        match states {
            Some(ss) => {
                Self::avoid_same_name(&ss)?;
                self.init_check_list(&ss, 0, &mut Default::default());
                self.state = Some(ss);
                self.is_state = true;
            }
            _ => {
                match &self.setting {
                    None => { self.is_state = false; }
                    Some(s) => {
                        if s.is_state {
                            self.is_state = true;
                        } else {
                            self.is_state = false;
                        }
                    }
                }
                self.state = None
            }
        }
        Ok(())
    }

    fn init_check_list(&mut self, ss: &States, id: u16, path: &mut StatePath) {
        let mut id = id;
        ss.iter().for_each(|s| {
            id += 1;
            match s {
                State::Normal(name) => {
                    let mut new = path.clone();
                    new.desc_seq.insert(0, CheckType::Normal(id));
                    self.check_list.insert(name.to_string(), new);
                }
                State::Parent(_, nss) => {
                    let mut new = path.clone();
                    new.desc_seq.insert(0, CheckType::Parent(id));
                    self.init_check_list(nss, id, &mut new);
                }
                State::Mutex(nss) => {
                    let mut new = path.clone();
                    new.is_mutex = true;
                    new.desc_seq.insert(0, CheckType::Mutex(id));
                    self.init_check_list(nss, id, &mut new);
                }
            }
        })
    }

    /// return.0 remained return.1 mutex pairs.
    pub fn check_state(&self, input: &Vec<String>) -> Result<(Vec<String>, Vec<(String, String)>)> {
        if !self.is_state {
            return Err(NatureError::VerifyError(format!("[{}] is not a state meta", self.meta_string())));
        }
        let mut map: HashMap<u16, (u16, String)> = HashMap::new();
        let mut remained: HashSet<String> = HashSet::new();
        let mut mutex_pairs: Vec<(String, String)> = vec![];
        for one in input {
            let option = self.check_list.get(one);
            // undefined
            if option.is_none() {
                let msg = format!("[{}] does not defined in meta: {}", one, self.meta_string());
                warn!("{}", &msg);
                return Err(NatureError::VerifyError(msg));
            }
            // not mutex
            let path = option.unwrap();
            if !path.is_mutex {
                remained.insert(one.clone());
                continue;
            }
            // mutex
            let mut last: u16 = 0;
            for op in &path.desc_seq {
                match op {
                    CheckType::Normal(id) => { last = *id; }
                    CheckType::Parent(id) => { last = *id; }
                    CheckType::Mutex(id) => {
                        let cached_p = map.get(id);
                        if let Some((e, old)) = cached_p {
                            if *e != last {
                                mutex_pairs.push((one.clone(), old.clone()));
                                remained.remove(old);
                                map.insert(*id, (last, one.clone()));
                            }
                            remained.insert(one.clone());
                        } else {
                            map.insert(*id, (last, one.clone()));
                            remained.insert(one.clone());
                            last = *id;
                        }
                    }
                }
            }
        }
        let remained: Vec<String> = remained.into_iter().collect();
        Ok((remained, mutex_pairs))
    }

    pub fn get_states(&self) -> Option<States> {
        self.state.clone()
    }
    pub fn is_state(&self) -> bool {
        self.is_state
    }

    fn avoid_same_name(s: &States) -> Result<()> {
        let mut set: HashSet<String> = HashSet::new();
        for one in s {
            let result = set.insert(one.get_name());
            if !result {
                return Err(NatureError::VerifyError(format!("repeated state name: [{}]", one.get_name())));
            }
        }
        Ok(())
    }

    /// before call this to make sure states had initialized.
    pub fn set_setting(&mut self, settings: &str) -> Result<()> {
        if !settings.is_empty() {
            let setting = MetaSetting::from_str(settings)?;
            if setting.is_state {
                self.is_state = true;
            }
            self.setting = Some(setting);
        } else {
            self.setting = None;
        }
        Ok(())
    }

    pub fn get_setting(&self) -> Option<MetaSetting> {
        self.setting.clone()
    }

    pub fn need_cache(&self) -> bool {
        match self.get_setting() {
            Some(setting) => setting.cache_saved,
            None => false
        }
    }

    pub fn check_master(&self, meta: &str) -> bool {
        match self.get_setting() {
            Some(setting) => match setting.master {
                Some(master) => master.eq(meta),
                None => false
            }
            None => false
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn key_can_not_be_null() {
        let key = String::new();
        let rtn = Meta::new(&key, 1, MetaType::Business);
        if let Err(NatureError::VerifyError(x)) = rtn {
            assert_eq!(x, "key length can't be zero");
        } else {
            panic!("should get error")
        }

        let key = "/".to_string();
        let rtn = Meta::new(&key, 1, MetaType::Business);
        if let Err(NatureError::VerifyError(x)) = rtn {
            assert_eq!(x, "key length can't be zero");
        } else {
            panic!("should get error")
        }
    }

    #[test]
    fn key_can_not_contain_colon() {
        let result = Meta::key_standardize("a:b");
        assert_eq!(result, Err(NatureError::VerifyError("key can not contains [:] character".to_string())));
    }

    #[test]
    fn key_can_be_empty_except_for_null_meta_type() {
        // key is empty
        let meta = Meta::new("", 1, MetaType::Null).unwrap();
        assert_eq!(MetaType::Null, meta.get_meta_type());
        assert_eq!(meta.meta_string(), "N::1");

        // key is not empty
        let meta = Meta::new("not empty", 1, MetaType::Null).unwrap();
        assert_eq!(MetaType::Null, meta.get_meta_type());
        assert_eq!(meta.meta_string(), "N::1");
    }

    /// also test for removing last separator and Business prefix
    #[test]
    fn standardize_no_separator_at_beginning() {
        println!("----------------- standardize_no_separator_at_beginning --------------------");
        let key = "a/b/c/".to_string();
        let rtn = Meta::new(&key, 1, MetaType::Business);
        assert_eq!("a/b/c", rtn.unwrap().key);
        let rtn = Meta::new(&key, 1, MetaType::Business);
        assert_eq!("B:a/b/c:1", rtn.unwrap().meta_string());
    }

    #[test]
    fn get_full_key() {
        println!("----------------- standardize_no_separator_at_beginning --------------------");
        let key = "a/b/c/".to_string();
        let rtn = Meta::new(&key, 1, MetaType::System);
        assert_eq!(rtn.unwrap().meta_string(), "S:a/b/c:1");
        let rtn = Meta::new(&key, 1, MetaType::Dynamic);
        assert_eq!(rtn.unwrap().meta_string(), "D:a/b/c:1");
        let rtn = Meta::new(&key, 1, MetaType::Business);
        assert_eq!(rtn.unwrap().meta_string(), "B:a/b/c:1");
        let rtn = Meta::new(&key, 1, MetaType::Null);
        assert_eq!(rtn.unwrap().meta_string(), "N::1");
    }

    #[test]
    fn from_meta_str() {
        // error full_key
        assert_eq!(Meta::from_string(":1"), Err(NatureError::VerifyError("format should be [MetaType]:[key]:[version]".to_string())));
        assert_eq!(Meta::from_string("s:1"), Err(NatureError::VerifyError("format should be [MetaType]:[key]:[version]".to_string())));
        assert_eq!(Meta::from_string("ss:1"), Err(NatureError::VerifyError("format should be [MetaType]:[key]:[version]".to_string())));
        assert_eq!(Meta::from_string("s:s:1"), Err(NatureError::VerifyError("unknow prefix : [s]".to_string())));
        assert_eq!(Meta::from_string("N::1"), Meta::new("", 1, MetaType::Null));
        assert_eq!(Meta::from_string("Na:1"), Err(NatureError::VerifyError("format should be [MetaType]:[key]:[version]".to_string())));
        assert_eq!(Meta::from_string("N:a:1"), Meta::new("a", 1, MetaType::Null));
        assert_eq!(Meta::from_string("D:hello:1"), Meta::new("hello", 1, MetaType::Dynamic));
        assert_eq!(Meta::from_string("S:world:1"), Meta::new("world", 1, MetaType::System));
        assert_eq!(Meta::from_string("B:my:1"), Meta::new("my", 1, MetaType::Business));
    }

    #[test]
    fn has_state_name_test() {
        let mut m = Meta::new("hello", 1, MetaType::Business).unwrap();
        assert_eq!(m.has_state_name("a"), false);
        let _ = m.set_states(Some(vec![State::Normal("a".to_string())]));
        assert_eq!(m.has_state_name("a"), true);
        assert_eq!(m.has_state_name("b"), false);
    }

    #[test]
    fn meta_string_test() {
        let m = Meta::new("hello", 1, MetaType::Business).unwrap();
        assert_eq!(m.meta_string(), "B:hello:1");
    }

    #[test]
    fn check_master_test() {
        let mut meta = Meta::default();
        assert_eq!(meta.check_master(""), false);
        assert_eq!(meta.check_master("abc"), false);
        let mut setting = MetaSetting::default();
        meta.setting = Some(setting.clone());
        assert_eq!(meta.check_master(""), false);
        assert_eq!(meta.check_master("abc"), false);
        setting.master = Some("def".to_string());
        meta.setting = Some(setting.clone());
        assert_eq!(meta.check_master(""), false);
        assert_eq!(meta.check_master("abc"), false);
        assert_eq!(meta.check_master("def"), true);
    }
}

#[cfg(test)]
mod verify_test {
    use super::*;

    #[test]
    fn not_a_state_meta() {
        let meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        let rtn = meta.check_state(&vec![]);
        assert_eq!(rtn, Err(NatureError::VerifyError("[B:hello:1] is not a state meta".to_string())))
    }

    #[test]
    fn none_states() {
        let mut meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        let setting = MetaSetting {
            is_state: true,
            master: None,
            multi_meta: Default::default(),
            cache_saved: false,
            only_one: false,
        }.to_json().unwrap();
        let _ = meta.set_setting(&setting);
        let set: Vec<String> = vec!["a".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn, Err(NatureError::VerifyError("[a] does not defined in meta: B:hello:1".to_string())))
    }

    #[test]
    fn simple() {
        let mut meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        let _ = match State::string_to_states("a") {
            Ok((ss, _)) => meta.set_states(Some(ss)),
            _ => { panic!("should have some") }
        };
        let set: Vec<String> = vec!["a".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn, Ok((vec!["a".to_string()], vec![])))
    }

    #[test]
    fn pure_parent() {
        let mut meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        let _ = match State::string_to_states("a1,a2,p1[a3,p2[p3[a,b,c]]]") {
            Ok((ss, _)) => meta.set_states(Some(ss)),
            _ => { panic!("should have some") }
        };
        let set = vec!["d".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn.is_err(), true);
        let set = vec!["b".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn, Ok((vec!["b".to_string()], vec![])));
    }

    #[test]
    fn simple_mutex() {
        let mut meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        let _ = match State::string_to_states("a|b") {
            Ok((ss, _)) => meta.set_states(Some(ss)),
            _ => { panic!("should have some") }
        };
        let set = vec!["b".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn, Ok((vec!["b".to_string()], vec![])));
        let set = vec!["b".to_string(), "a".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn, Ok((vec!["a".to_string()], vec![("a".to_string(), "b".to_string())])));
    }

    #[test]
    fn parent_in_mutex() {
        let mut meta = Meta::new("/hello", 1, MetaType::Business).unwrap();
        let _ = match State::string_to_states("a|b[c|d,e]]") {
            Ok((ss, _)) => meta.set_states(Some(ss)),
            _ => { panic!("should have some") }
        };
        let set = vec!["a".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn, Ok((vec!["a".to_string()], vec![])));

        let set = vec!["a".to_string(), "c".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn, Ok((vec!["c".to_string()], vec![("c".to_string(), "a".to_string())])));

        let set = vec!["a".to_string(), "d".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn, Ok((vec!["d".to_string()], vec![("d".to_string(), "a".to_string())])));

        let set = vec!["c".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn, Ok((vec!["c".to_string()], vec![])));

        let set = vec!["c".to_string(), "d".to_string()];
        let rtn = meta.check_state(&set);
        assert_eq!(rtn, Ok((vec!["d".to_string()], vec![("d".to_string(), "c".to_string())])));

        let set = vec!["c".to_string(), "e".to_string()];
        let rtn = meta.check_state(&set);
        let rtn: Vec<String> = rtn.unwrap().0;
        assert_eq!(rtn.len(), 2);
        assert_eq!(rtn.contains(&"c".to_string()), true);
        assert_eq!(rtn.contains(&"e".to_string()), true);
    }
}
