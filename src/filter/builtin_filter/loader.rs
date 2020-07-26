use std::str::FromStr;
use std::sync::Arc;

use nature_common::{Executor, get_para_part, Instance, KeyCondition, NatureError, Result};
use nature_db::KeyRange;

use crate::filter::builtin_filter::FilterBefore;
use crate::filter::filter_before;

pub struct Loader {
    pub dao: Arc<dyn KeyRange>
}

/// **notice** can only load one page data! more page consider use with `MetaType::Loop`
#[async_trait]
impl FilterBefore for Loader {
    async fn filter(&self, ins: &mut Instance, cfg: &str) -> Result<()> {
        let setting = Setting::get(&cfg)?;
        let time_range = match get_para_part(&ins.para, &setting.time_part) {
            Ok(rtn) => rtn,
            Err(e) => {
                let msg = format!("built-in::Loader instance's para has no time info: {}", e.to_string());
                return Err(NatureError::VerifyError(msg));
            }
        };
        let mut condition = KeyCondition {
            id: "".to_string(),
            meta: "".to_string(),
            key_gt: setting.key_gt,
            key_ge: "".to_string(),
            key_lt: setting.key_lt,
            key_le: "".to_string(),
            para: "".to_string(),
            state_version: 0,
            time_ge: Some(i64::from_str(&time_range[0])?),
            time_lt: Some(i64::from_str(&time_range[1])?),
            limit: setting.page_size as i32,
        };
        let mut content: Vec<String> = vec![];
        let rtn: Vec<Instance> = self.dao.get_by_key_range(&condition).await?;
        let len = rtn.len();
        if len == setting.page_size as usize {
            condition.key_gt = rtn[len - 1].get_key();
        }
        for mut one in rtn {
            filter_before(&mut one, setting.filters.clone()).await?;
            content.push(one.content.to_string());
        }
        ins.content = serde_json::to_string(&content)?;
        Ok(())
    }
}


/// when used this mode the target `MetaType` must be `Multi`
#[derive(Serialize, Deserialize, Debug)]
struct Setting {
    /// great than `ins_key`
    key_gt: String,
    /// less equal `ins_key`
    key_lt: String,
    #[serde(skip_serializing_if = "is_100")]
    #[serde(default = "default_100")]
    page_size: u16,
    /// where to get the time range from the `Instance'para` which used to load data from Instance table
    /// it only accept two element, one for Begin Time and the other for End Time
    time_part: Vec<u8>,
    /// correct the format of the data loaded.
    filters: Vec<Executor>,
}

impl Setting {
    pub fn get(cfg: &str) -> Result<Setting> {
        if cfg.is_empty() {
            return Err(NatureError::VerifyError("builtin-filter loader `settings` can't be empty".to_string()));
        };
        let result: Setting = serde_json::from_str(cfg)?;
        if result.time_part.len() != 2 {
            return Err(NatureError::VerifyError("builtin-filter loader `settings.time_part` need exactly 2 elements".to_string()));
        }
        Ok(result)
    }
}

fn is_100(size: &u16) -> bool {
    if *size == 100 {
        true
    } else {
        false
    }
}

fn default_100() -> u16 { 100 }

#[cfg(test)]
mod loader_test {
    use super::*;

    #[tokio::test]
    // #[ignore]
    async fn with_sub_filter() {
        let loader = Loader { dao: Arc::new(Mocker {}) };
        let mut instance = Instance::default();
        instance.para = "123/456".to_string();
        instance.content = "lxb".to_string();
        let setting = r#"{"key_gt":"abc","key_lt":"def","time_part":[0,1],"filters":[
            {"protocol":"localRust","url":"nature_integrate_test_executor:append_star"},
            {"protocol":"localRust","url":"nature_integrate_test_executor:append_plus"}
        ]}"#;
        let _rtn = loader.filter(&mut instance, setting).await;
        assert_eq!("[\"one * +\",\"two * +\"]", instance.content);
    }

    #[tokio::test]
    async fn no_sub_filter() {
        let loader = Loader { dao: Arc::new(Mocker {}) };
        let mut instance = Instance::default();
        instance.para = "123/456".to_string();
        instance.content = "lxb".to_string();
        let setting = r#"{"key_gt":"abc","key_lt":"def","time_part":[0,1],"filters":[]}"#;
        let _ = loader.filter(&mut instance, setting).await;
        assert_eq!("[\"one\",\"two\"]", instance.content);
    }

    #[tokio::test]
    async fn instance_para_not_set() {
        let loader = Loader { dao: Arc::new(Mocker {}) };
        let mut instance = Instance::default();
        instance.content = "lxb".to_string();
        let setting = r#"{"key_gt":"abc","key_lt":"def","time_part":[1,2],"filters":[]}"#;
        let err = loader.filter(&mut instance, setting).await.err().unwrap();
        assert_eq!(true, err.to_string().contains("built-in"));
    }

    struct Mocker;

    #[async_trait]
    impl KeyRange for Mocker {
        async fn get_by_key_range(&self, _para: &KeyCondition) -> Result<Vec<Instance>> {
            let mut one = Instance::default();
            one.content = "one".to_string();
            let mut two = Instance::default();
            two.content = "two".to_string();
            Ok(vec![one, two])
        }
    }
}

#[cfg(test)]
mod setting_test {
    use super::*;

    #[test]
    fn setting_is_ok() {
        let s = r#"{"key_gt":"abc","key_lt":"def","time_part":[1,2],"filters":[]}"#;
        let rtn = Setting::get(s);
        assert_eq!(true, rtn.is_ok());
    }

    #[test]
    fn time_part_more_than_two() {
        let s = r#"{"key_gt":"abc","key_lt":"def","time_part":[1,2,3],"filters":[]}"#;
        let err = Setting::get(s).err().unwrap();
        assert_eq!(NatureError::VerifyError("builtin-filter loader `settings.time_part` need exactly 2 elements".to_string()), err);
    }

    #[test]
    fn time_part_less_than_two() {
        let s = r#"{"key_gt":"abc","key_lt":"def","time_part":[1],"filters":[]}"#;
        let err = Setting::get(s).err().unwrap();
        assert_eq!(NatureError::VerifyError("builtin-filter loader `settings.time_part` need exactly 2 elements".to_string()), err);
    }

    #[test]
    fn filter_not_set() {
        let s = r#"{"key_gt":"abc","key_lt":"def","time_part":[1]}"#;
        let err = Setting::get(s).err().unwrap();
        assert_eq!(NatureError::VerifyError("missing field `filters` at line 1 column 47".to_string()), err);
    }

    #[test]
    fn time_part_not_set() {
        let s = r#"{"key_gt":"abc","key_lt":"def"}"#;
        let err = Setting::get(s).err().unwrap();
        assert_eq!(NatureError::VerifyError("missing field `time_part` at line 1 column 31".to_string()), err);
    }

    #[test]
    fn lt_not_set() {
        let err = Setting::get(r#"{"key_gt":"abc"}"#).err().unwrap();
        assert_eq!(NatureError::VerifyError("missing field `key_lt` at line 1 column 16".to_string()), err);
    }

    #[test]
    fn gt_not_set() {
        let err = Setting::get("{}").err().unwrap();
        assert_eq!(NatureError::VerifyError("missing field `key_gt` at line 1 column 2".to_string()), err);
    }

    #[test]
    fn empty() {
        let err = Setting::get("").err().unwrap();
        assert_eq!(NatureError::VerifyError("builtin-filter loader `settings` can't be empty".to_string()), err);
    }
}