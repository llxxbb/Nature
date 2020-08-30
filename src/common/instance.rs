use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;
use std::ops::{Deref, DerefMut};

use chrono::prelude::*;
use futures::Future;
use itertools::Itertools;

use crate::common::{DynamicConverter, FromInstance, generate_id, ID, is_default, KeyCondition, MetaType, NatureError, Result, SEPARATOR_INS_KEY, SEPARATOR_META, TargetState};

use super::Meta;

// sys context define
pub static CONTEXT_TARGET_INSTANCE_ID: &str = "target.id";
pub static CONTEXT_TARGET_INSTANCE_PARA: &str = "target.para";

pub static CONTEXT_LOOP_NEXT: &str = "loop.next";
pub static CONTEXT_LOOP_ID: &str = "loop.id";
pub static CONTEXT_LOOP_TASK: &str = "loop.task";
pub static CONTEXT_LOOP_FINISHED: &str = "loop.finished";

pub static CONTEXT_DYNAMIC_PARA: &str = "para.dynamic";

/// A snapshot for a particular `Meta`
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Instance {
    /// A unique value used to distinguish other instance
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub id: ID,
    pub data: BizObject,
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
            data: BizObject {
                meta: format!("{}{}{}{}1", MetaType::default().get_prefix(), *SEPARATOR_META, key, *SEPARATOR_META),
                content: "".to_string(),
                context: HashMap::new(),
                sys_context: HashMap::new(),
                states: HashSet::new(),
                state_version: 0,
                from: None,
                para: String::new(),
            },
            create_time: 0,
        })
    }

    pub fn revise(&mut self) -> Result<&mut Self> {
        self.create_time = Local::now().timestamp_millis();
        if self.para.is_empty() && self.id == 0 {
            self.id = generate_id(&self.data)?;
        }
        Ok(self)
    }

    pub fn meta_must_same(is: &Vec<Self>) -> Result<()> {
        if is.len() < 2 {
            return Ok(());
        }
        let option = is[1..].iter().find(|x| { !x.meta.eq(&is[0].meta) });
        match option {
            Some(_) => Err(NatureError::VerifyError("instances's meta must be same!".to_string())),
            None => Ok(())
        }
    }

    pub async fn get_master<'a, F, ID>(&self, self_meta: &Meta, dao: ID) -> Result<Option<Instance>>
        where F: Future<Output=Result<Option<Instance>>>,
              ID: Fn(KeyCondition) -> F
    {
        match self_meta.get_setting() {
            None => Ok(None),
            Some(setting) => match setting.master {
                None => Ok(None),
                Some(master) => {
                    let condition = KeyCondition::new(self.id, &master, &self.para, 0);
                    let result = dao(condition);
                    Ok(result.await?)
                }
            },
        }
    }

    pub fn get_key(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}{:x}{}{}{}{}", self.meta, sep, self.id, sep, self.para, sep, self.state_version)
    }

    pub fn key_no_state(&self) -> String {
        let sep: &str = &*SEPARATOR_INS_KEY;
        format!("{}{}{:x}{}{}", self.meta, sep, self.id, sep, self.para)
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

impl Into<KeyCondition> for Instance {
    fn into(self) -> KeyCondition {
        KeyCondition {
            id: format!("{:x}", self.id),
            meta: self.data.meta.to_string(),
            key_gt: "".to_string(),
            key_ge: "".to_string(),
            key_lt: "".to_string(),
            key_le: "".to_string(),
            para: self.data.para.to_string(),
            state_version: self.data.state_version,
            time_ge: None,
            time_lt: None,
            limit: 1,
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
    /// This instance's Type
    pub meta: String,
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
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub state_version: i32,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub from: Option<FromInstance>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub para: String,
}

impl Hash for BizObject {
    fn hash<H: Hasher>(&self, s: &mut H) {
        self.meta.hash(s);
        self.content.hash(s);
        self.state_version.hash(s);
        self.from.hash(s);
        self.para.hash(s);
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
    pub fn modify_state(&mut self, add_and_delete: &TargetState, meta: &Meta) {
        // delete first
        if let Some(x) = &add_and_delete.remove {
            x.iter().for_each(|one| { self.states.remove(one); });
        }
        let mut append: Vec<String> = self.states.clone().into_iter().collect();
        match &add_and_delete.add {
            Some(ss) => {
                append.append(&mut ss.clone());
                let (remained, _) = meta.check_state(&append).unwrap();
                self.states = remained.into_iter().collect();
            }
            None => ()
        }
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
            data: self.instance.data.clone(),
            create_time: 0,
        }
    }

    pub fn get_key(&self) -> String {
        self.instance.get_key()
    }
}

#[cfg(test)]
mod test {
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
        assert_eq!(ins.meta, "B:hello:1");
        let ins = Instance::new("/hello").unwrap();
        assert_eq!(ins.meta, "B:hello:1");
    }

    #[test]
    fn instance_json_test() {
        let mut order = Instance::new("sale/order").unwrap();
        order.data.content = "my order detail".to_string();
        let rtn = serde_json::to_string(&order).unwrap();
        assert_eq!(rtn, r#"{"data":{"meta":"B:sale/order:1","content":"my order detail"}}"#);
    }
}



