use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};

use chrono::prelude::*;
use futures::Future;
use itertools::Itertools;
use crate::common::*;

use crate::db::relation_target::RelationTarget;
use crate::domain::*;
use crate::util::*;

// sys context define
pub static CONTEXT_TARGET_INSTANCE_ID: &str = "target.id";
pub static CONTEXT_TARGET_INSTANCE_PARA: &str = "target.para";

pub static CONTEXT_LOOP_NEXT: &str = "loop.next";
pub static CONTEXT_LOOP_ID: &str = "loop.id";
pub static CONTEXT_LOOP_TASK: &str = "loop.task";
pub static CONTEXT_LOOP_FINISHED: &str = "loop.finished";

pub static CONTEXT_DYNAMIC_PARA: &str = "para.dynamic";

/// A snapshot for a particular `Meta`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Instance {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub id: u64,
    pub path: Modifier,
    /// data Nature can't controlled
    pub data: BizObject,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub from: Option<InstanceLocator>,
    /// When this instance created in db
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub create_time: i64,
}


impl Instance {
    pub fn new(key: &str) -> Result<Self> {
        if key.is_empty() {
            return Err(NatureError::VerifyError("key can not be empty".to_string()));
        }
        let key = Meta::key_standardize(key)?;
        Ok(Instance {
            id: 0,
            path: Modifier {
                meta: format!("{}{}{}{}1", MetaType::default().get_prefix(), *SEPARATOR_META, key, *SEPARATOR_META),
                para: "".to_string(),
                state_version: 0,
            },
            data: BizObject {
                content: "".to_string(),
                context: HashMap::new(),
                sys_context: HashMap::new(),
                states: HashSet::new(),
            },
            from: None,
            create_time: 0,
        })
    }

    pub fn revise(&mut self) -> Result<&mut Self> {
        self.create_time = Local::now().timestamp_millis();
        if self.path.para.is_empty() && self.id == 0 {
            self.id = generate_id(&self.data)?;
        }
        Ok(self)
    }

    pub fn meta_must_same(is: &Vec<Self>) -> Result<()> {
        if is.len() < 2 {
            return Ok(());
        }
        let option = is[1..].iter().find(|x| { !x.path.meta.eq(&is[0].path.meta) });
        match option {
            Some(_) => Err(NatureError::VerifyError("instances meta must be same!".to_string())),
            None => Ok(())
        }
    }

    pub async fn get_master<'a, F, ID>(&self, self_meta: &Meta, dao: ID) -> Result<Option<Instance>>
        where F: Future<Output=Result<Option<Instance>>>,
              ID: Fn(InsCond) -> F
    {
        match self_meta.get_setting() {
            None => Ok(None),
            Some(setting) => match setting.master {
                None => Ok(None),
                Some(master) => {
                    let condition = InsCond::new(self.id, &master, &self.path.para, 0);
                    let result = dao(condition);
                    Ok(result.await?)
                }
            },
        }
    }

    pub fn get_key(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}{}{}{}{}{}", self.path.meta, sep, self.id, sep, self.path.para, sep, self.path.state_version)
    }

    pub fn key_no_state(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}{}{}{}", self.path.meta, sep, self.id, sep, self.path.para)
    }

    pub fn init_meta(meta_setting: &MetaSetting, instances: &mut Vec<Instance>, from: &InstanceLocator) -> Result<()> {
        // when has one then use it.
        if meta_setting.multi_meta.len() == 1 {
            let meta = meta_setting.multi_meta.iter().next().unwrap();
            for instance in instances {
                instance.path.meta = meta.to_string();
                instance.from = Some(from.clone());
            }
            return Ok(());
        }
        // otherwise check each meta
        for instance in instances {
            if !meta_setting.multi_meta.contains(&instance.path.meta) {
                let msg = format!("undefined meta:{} ", instance.path.meta);
                return Err(NatureError::VerifyError(msg));
            }
            instance.from = Some(from.clone());
        }
        Ok(())
    }
}

impl Hash for Instance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.data.hash(state);
        self.from.hash(state);
        self.create_time.hash(state);
    }
}

impl Deref for Instance {
    type Target = BizObject;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.data
    }
}

impl DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Into<InsCond> for Instance {
    fn into(self) -> InsCond {
        InsCond {
            id: self.id,
            time_ge: None,
            time_lt: None,
            other: NoIdCond {
                meta: self.path.meta.to_string(),
                key_gt: "".to_string(),
                key_ge: "".to_string(),
                key_lt: "".to_string(),
                key_le: "".to_string(),
                para: self.path.para.to_string(),
                state_version: self.path.state_version,
                limit: 1,
            }
        }
    }
}

impl Iterator for Instance {
    type Item = Instance;
    fn next(&mut self) -> Option<Self::Item> {
        let rtn: &Instance = self;
        Some(rtn.clone())
    }
}

/// A snapshot for a particular `Meta`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct BizObject {
    /// What contend in this instance for the `Meta`
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub content: String,
    /// Is a json for a `Map[key, value]` which maybe used for next `Relation`
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub context: HashMap<String, String>,
    /// like `context` but is specified by Nature
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub sys_context: HashMap<String, String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub states: HashSet<String>,
}

impl Hash for BizObject {
    fn hash<H: Hasher>(&self, s: &mut H) {
        self.content.hash(s);
        self.context.iter().sorted().for_each(|one| {
            one.0.hash(s);
            one.1.hash(s)
        });
        self.sys_context.iter().sorted().for_each(|one| {
            one.0.hash(s);
            one.1.hash(s);
        });
        self.states.iter().sorted().for_each(|one| one.hash(s));
    }
}

impl BizObject {
    pub fn modify_state(&mut self, add_and_delete: &RelationTarget, meta: &Meta) {
        // delete first
        add_and_delete.state_remove.iter().for_each(|one| { self.states.remove(one); });
        let mut append: Vec<String> = self.states.clone().into_iter().collect();
        let ss = &add_and_delete.state_add;
        if ss.is_empty() {
            return;
        }
        append.append(&mut ss.clone());
        let (remained, _) = meta.check_state(&append).unwrap();
        self.states = remained.into_iter().collect();
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct SelfRouteInstance {
    pub instance: Instance,
    pub converter: Vec<DynamicConverter>,
}

impl SelfRouteInstance {
    pub fn verify(&self) -> Result<()> {
        if self.converter.is_empty() {
            return Err(NatureError::VerifyError("executor must not empty for dynamic convert!".to_string()));
        }
        Ok(())
    }
    pub fn to_instance(&self) -> Instance {
        Instance {
            id: 0,
            path: self.instance.path.clone(),
            data: self.instance.data.clone(),
            from: None,
            create_time: 0,
        }
    }

    pub fn get_key(&self) -> String {
        self.instance.get_key()
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn same_meta_test() {
        let vec1 = vec![Instance::new("hello").unwrap(), Instance::new("world").unwrap()];
        assert_eq!(Instance::meta_must_same(&vec1).is_err(), true);
        let vec1 = vec![Instance::new("hello").unwrap(), Instance::new("hello").unwrap()];
        assert_eq!(Instance::meta_must_same(&vec1).is_err(), false);
    }

    #[test]
    fn instance_new_test() {
        let ins = Instance::new("hello").unwrap();
        assert_eq!(ins.path.meta, "B:hello:1");
        let ins = Instance::new("/hello").unwrap();
        assert_eq!(ins.path.meta, "B:hello:1");
    }

    #[test]
    fn instance_json_test() {
        let mut order = Instance::new("sale/order").unwrap();
        order.data.content = "my order detail".to_string();
        let rtn = serde_json::to_string(&order).unwrap();
        assert_eq!(rtn, r#"{"path":{"meta":"B:sale/order:1"},"data":{"content":"my order detail"}}"#);
    }

    #[test]
    fn json_ok_instance_test() {
        // instance
        let instance = Instance::new("hello").unwrap();
        let ok_ins: Result<Instance> = Ok(instance);
        let result = serde_json::to_string(&ok_ins).unwrap();
        println!("{}", result);
        let result: Result<Instance> = serde_json::from_str(&result).unwrap();
        assert_eq!(result, ok_ins)
    }

    #[test]
    fn json_err_test() {
        // instance
        let ok_ins: Result<Instance> = Err(NatureError::LogicalError("hello".to_string()));
        let result = serde_json::to_string(&ok_ins).unwrap();
        println!("{}", result);
        let result: Result<Instance> = serde_json::from_str(&result).unwrap();
        assert_eq!(result, ok_ins)
    }

    #[test]
    fn check_multi_meta() {
        let mut set: BTreeSet<String> = BTreeSet::new();
        set.insert("B:a:1".to_string());
        set.insert("B:b:1".to_string());

        let ms = MetaSetting {
            name: None,
            is_state: false,
            master: None,
            multi_meta: set,
            cache_saved: false,
            only_one: false,
        };
        let a = Instance::new("a").unwrap();
        let b = Instance::new("b").unwrap();
        let c = Instance::new("d").unwrap();
        assert_eq!(Instance::init_meta(&ms, &mut vec![a.clone()], &InstanceLocator::default()).is_ok(), true);
        assert_eq!(Instance::init_meta(&ms, &mut vec![b.clone()], &InstanceLocator::default()).is_ok(), true);
        assert_eq!(Instance::init_meta(&ms, &mut vec![a.clone(), b.clone()], &InstanceLocator::default()).is_ok(), true);
        assert_eq!(Instance::init_meta(&ms, &mut vec![c.clone()], &InstanceLocator::default()).is_err(), true);
        assert_eq!(Instance::init_meta(&ms, &mut vec![c.clone(), a.clone()], &InstanceLocator::default()).is_err(), true);
        assert_eq!(Instance::init_meta(&ms, &mut vec![a.clone(), c.clone()], &InstanceLocator::default()).is_err(), true);
        assert_eq!(Instance::init_meta(&ms, &mut vec![b.clone(), c.clone()], &InstanceLocator::default()).is_err(), true);
        assert_eq!(Instance::init_meta(&ms, &mut vec![c.clone(), b.clone()], &InstanceLocator::default()).is_err(), true);
        assert_eq!(Instance::init_meta(&ms, &mut vec![a, b, c], &InstanceLocator::default()).is_err(), true);
    }

    #[test]
    fn set_meta_for_multi_meta() {
        let mut set: BTreeSet<String> = BTreeSet::new();
        set.insert("B:a:1".to_string());

        let ms = MetaSetting {
            name: None,
            is_state: false,
            master: None,
            multi_meta: set,
            cache_saved: false,
            only_one: false,
        };
        let a = Instance::default();
        let b = Instance::default();
        let c = Instance::default();
        let ins = &mut vec![a, b, c];
        let _ = Instance::init_meta(&ms, ins, &InstanceLocator::default());
        assert_eq!("B:a:1", ins[0].path.meta);
        assert_eq!("B:a:1", ins[1].path.meta);
        assert_eq!("B:a:1", ins[2].path.meta);
    }
}



