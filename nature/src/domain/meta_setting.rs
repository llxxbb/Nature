use std::collections::btree_set::BTreeSet;
use std::str::FromStr;

use crate::domain::*;
use crate::util::*;

#[derive(Debug, Clone, Default, PartialEq, Ord, PartialOrd, Eq)]
#[derive(Serialize, Deserialize)]
pub struct MetaSetting {
    /// friendly name for `Meta`
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub is_state: bool,
    /// Only useful for state-meta.
    /// A meta_string, this meta instance's id will use its master instance's id.
    /// As a target meta, if no `executor` appointed. an auto-converter will be created.
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub master: Option<String>,
    /// `MetaSettingTemp#multi_meta` can't use BTreeSet type,
    /// so make this struct for it,
    /// it would be good performance for multi_meta verify.
    /// each of the item's format is meta-type:business-key:version
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub multi_meta: BTreeSet<String>,
    /// Nature will cache the saved instance for a while, this can increase performance greatly to save same instance, such as to generate a timer instance.
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub cache_saved: bool,
    /// only used by `MetaType::Loop`, has only one instance generated when loop finished.
    /// Requirement: multi_meta should has only one item
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub only_one: bool,
}

impl FromStr for MetaSetting {
    type Err = NatureError;

    fn from_str(s: &str) -> Result<Self> {
        let tmp: MetaSetting = serde_json::from_str(s)?;
        tmp.check_sub_meta()?;
        Ok(tmp)
    }
}

impl MetaSetting {
    pub fn set_instance_meta(&self, instances: &mut Vec<Instance>, from: &FromInstance) -> Result<()> {
        // when has one then use it.
        if self.multi_meta.len() == 1 {
            let meta = self.multi_meta.iter().next().unwrap();
            for instance in instances {
                instance.meta = meta.to_string();
                instance.from = Some(from.clone());
            }
            return Ok(());
        }
        // otherwise check each meta
        for instance in instances {
            if !self.multi_meta.contains(&instance.meta) {
                let msg = format!("undefined meta:{} ", instance.meta);
                return Err(NatureError::VerifyError(msg));
            }
            instance.from = Some(from.clone());
        }
        Ok(())
    }

    pub fn to_json(&self) -> Result<String> {
        let rtn = serde_json::to_string(&self)?;
        Ok(rtn)
    }

    /// sub `Meta` can't be Loop or Multi
    pub fn check_sub_meta(&self) -> Result<()> {
        let err = self.multi_meta.iter().any(|one| {
            MetaType::check_type(one, MetaType::Loop).is_ok()
                || MetaType::check_type(one, MetaType::Multi).is_ok()
        });
        if err {
            return Err(NatureError::VerifyError("MetaType: Multi or Loop can't be as sub meta".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sub_meta_test() {
        // loop check
        let mut set = MetaSetting::default();
        set.multi_meta.insert("L:myErr".to_string());
        let result = set.check_sub_meta();
        assert_eq!(true, result.is_err());

        // multi check
        let mut set = MetaSetting::default();
        set.multi_meta.insert("M:myErr".to_string());
        let result = set.check_sub_meta();
        assert_eq!(true, result.is_err());

        // biz check
        let mut set = MetaSetting::default();
        set.multi_meta.insert("B:myErr".to_string());
        let result = set.check_sub_meta();
        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn name_test() {
        let mut set = MetaSetting::default();
        set.name = Some("hi".to_string());
        let json = serde_json::to_string(&set).unwrap();
        assert_eq!(r#"{"name":"hi"}"#, json);
        let f_json: MetaSetting = serde_json::from_str(&json).unwrap();
        assert_eq!(f_json, set)
    }

    #[test]
    fn master_test() {
        let mut set = MetaSetting::default();
        set.master = Some("B:from:1".to_string());
        let result = serde_json::to_string(&set).unwrap();
        assert_eq!(result, r#"{"master":"B:from:1"}"#)
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
        assert_eq!(ms.set_instance_meta(&mut vec![a.clone()], &FromInstance::default()).is_ok(), true);
        assert_eq!(ms.set_instance_meta(&mut vec![b.clone()], &FromInstance::default()).is_ok(), true);
        assert_eq!(ms.set_instance_meta(&mut vec![a.clone(), b.clone()], &FromInstance::default()).is_ok(), true);
        assert_eq!(ms.set_instance_meta(&mut vec![c.clone()], &FromInstance::default()).is_err(), true);
        assert_eq!(ms.set_instance_meta(&mut vec![c.clone(), a.clone()], &FromInstance::default()).is_err(), true);
        assert_eq!(ms.set_instance_meta(&mut vec![a.clone(), c.clone()], &FromInstance::default()).is_err(), true);
        assert_eq!(ms.set_instance_meta(&mut vec![b.clone(), c.clone()], &FromInstance::default()).is_err(), true);
        assert_eq!(ms.set_instance_meta(&mut vec![c.clone(), b.clone()], &FromInstance::default()).is_err(), true);
        assert_eq!(ms.set_instance_meta(&mut vec![a, b, c], &FromInstance::default()).is_err(), true);
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
        let _ = ms.set_instance_meta(ins, &FromInstance::default());
        assert_eq!("B:a:1", ins[0].meta);
        assert_eq!("B:a:1", ins[1].meta);
        assert_eq!("B:a:1", ins[2].meta);
    }

    #[test]
    fn cache_saved_test() {
        let setting = r#"{"cache_saved":true}"#;
        let result: MetaSetting = serde_json::from_str(&setting).unwrap();
        let result = MetaSetting::from(result);
        assert_eq!(result.cache_saved, true);
    }

    #[test]
    fn get_master_test() {
        let mut setting = MetaSetting {
            name: None,
            is_state: false,
            master: Some("abc".to_string()),
            multi_meta: Default::default(),
            cache_saved: false,
            only_one: false,
        };
        let mut m = Meta::from_string("B:test:3").unwrap();
        let _ = m.set_setting(&setting.to_json().unwrap());
        let rtn = m.get_setting().unwrap().master.unwrap();
        assert_eq!("abc", rtn);

        // for none
        setting.master = None;
        let _ = m.set_setting(&setting.to_json().unwrap());
        let rtn = m.get_setting().unwrap().master;
        assert_eq!(None, rtn);
    }

    #[test]
    fn get_sub_test() {
        let mut set: BTreeSet<String> = BTreeSet::new();
        set.insert("B:p/a:1".to_owned());
        set.insert("B:p/b:1".to_owned());
        let setting = MetaSetting {
            name: None,
            is_state: false,
            master: None,
            multi_meta: set,
            cache_saved: false,
            only_one: false,
        };
        let mut m = Meta::from_string("B:test:3").unwrap();
        let _ = m.set_setting(&setting.to_json().unwrap());
        let rtn = m.get_setting().unwrap().multi_meta;
        assert_eq!(2, rtn.len());
        assert_eq!(true, rtn.contains(&"B:p/a:1".to_string()));
        assert_eq!(true, rtn.contains(&"B:p/b:1".to_string()));
    }
}