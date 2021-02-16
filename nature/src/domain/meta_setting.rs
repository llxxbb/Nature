use std::collections::btree_set::BTreeSet;
use std::str::FromStr;

use crate::domain::*;
use crate::util::*;

#[derive(Debug, Clone, Default, PartialEq, Ord, PartialOrd, Eq)]
#[derive(Serialize, Deserialize)]
pub struct MetaSetting {
    pub is_state: bool,
    /// Only useful for state-meta.
    /// A meta_string, this meta instance's id will use its master instance's id.
    /// As a target meta, if no `executor` appointed. an auto-converter will be created.
    pub master: Option<String>,
    /// `MetaSettingTemp#multi_meta` can't use BTreeSet type,
    /// so make this struct for it,
    /// it would be good performance for multi_meta verify.
    /// each of the item's format is meta-type:business-key:version
    pub multi_meta: BTreeSet<String>,
    /// Nature will cache the saved instance for a while, this can increase performance greatly to save same instance, such as to generate a timer instance.
    pub cache_saved: bool,
    /// only used by `MetaType::Loop`, has only one instance generated when loop finished.
    /// Requirement: multi_meta should has only one item
    pub only_one: bool,
}

impl From<MetaSettingTemp> for MetaSetting {
    fn from(input: MetaSettingTemp) -> Self {
        MetaSetting {
            is_state: input.is_state,
            master: input.master,
            multi_meta: {
                let mut rtn = BTreeSet::<String>::new();
                input.multi_meta.into_iter().for_each(|one| { rtn.insert(one); });
                rtn
            },
            cache_saved: input.cache_saved,
            only_one: input.only_one,
        }
    }
}

impl FromStr for MetaSetting {
    type Err = NatureError;

    fn from_str(s: &str) -> Result<Self> {
        let tmp: MetaSettingTemp = serde_json::from_str(s)?;
        Ok(tmp.into())
    }
}

impl From<MetaSetting> for MetaSettingTemp {
    fn from(input: MetaSetting) -> Self {
        MetaSettingTemp {
            is_state: input.is_state,
            master: input.master,
            multi_meta: {
                let mut rtn: Vec<String> = vec![];
                input.multi_meta.into_iter().for_each(|one| { rtn.push(one); });
                rtn
            },
            cache_saved: input.cache_saved,
            only_one: input.only_one,
        }
    }
}

impl MetaSetting {
    pub fn check_multi_meta(&self, instances: &mut Vec<Instance>, from: &FromInstance) -> Result<()> {
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
        let temp = MetaSettingTemp::from(self.clone());
        let rtn = serde_json::to_string(&temp)?;
        Ok(rtn)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
struct MetaSettingTemp {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub is_state: bool,
    /// Only useful for state-meta.
    /// A meta_string, this meta instance's id will use its master instance's id.
    /// As a target meta, if no `executor` appointed. an auto-converter will be created.
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub master: Option<String>,
    /// each of the item's format is meta-type:business-key:version
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub multi_meta: Vec<String>,
    /// Nature will cache the saved instance for a while, and check before saving the following same instances.
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub cache_saved: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub only_one: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn master_test() {
        let mut set = MetaSettingTemp::default();
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
            is_state: false,
            master: None,
            multi_meta: set,
            cache_saved: false,
            only_one: false,
        };
        let a = Instance::new("a").unwrap();
        let b = Instance::new("b").unwrap();
        let c = Instance::new("d").unwrap();
        assert_eq!(ms.check_multi_meta(&mut vec![a.clone()], &FromInstance::default()).is_ok(), true);
        assert_eq!(ms.check_multi_meta(&mut vec![b.clone()], &FromInstance::default()).is_ok(), true);
        assert_eq!(ms.check_multi_meta(&mut vec![a.clone(), b.clone()], &FromInstance::default()).is_ok(), true);
        assert_eq!(ms.check_multi_meta(&mut vec![c.clone()], &FromInstance::default()).is_err(), true);
        assert_eq!(ms.check_multi_meta(&mut vec![c.clone(), a.clone()], &FromInstance::default()).is_err(), true);
        assert_eq!(ms.check_multi_meta(&mut vec![a.clone(), c.clone()], &FromInstance::default()).is_err(), true);
        assert_eq!(ms.check_multi_meta(&mut vec![b.clone(), c.clone()], &FromInstance::default()).is_err(), true);
        assert_eq!(ms.check_multi_meta(&mut vec![c.clone(), b.clone()], &FromInstance::default()).is_err(), true);
        assert_eq!(ms.check_multi_meta(&mut vec![a, b, c], &FromInstance::default()).is_err(), true);
    }

    #[test]
    fn set_meta_for_multi_meta() {
        let mut set: BTreeSet<String> = BTreeSet::new();
        set.insert("B:a:1".to_string());

        let ms = MetaSetting {
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
        let _ = ms.check_multi_meta(ins, &FromInstance::default());
        assert_eq!("B:a:1", ins[0].meta);
        assert_eq!("B:a:1", ins[1].meta);
        assert_eq!("B:a:1", ins[2].meta);
    }

    #[test]
    fn cache_saved_test() {
        let setting = r#"{"cache_saved":true}"#;
        let result: MetaSettingTemp = serde_json::from_str(&setting).unwrap();
        let result = MetaSetting::from(result);
        assert_eq!(result.cache_saved, true);
    }
}