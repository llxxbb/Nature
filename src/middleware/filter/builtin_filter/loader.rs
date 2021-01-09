use std::str::FromStr;
use std::sync::Arc;

use crate::db::KeyRange;
use crate::domain::*;
use crate::middleware::filter::builtin_filter::FilterBefore;
use crate::middleware::filter::convert_before;
use crate::util::*;

pub struct Loader {
    pub dao: Arc<dyn KeyRange>
}

/// **notice** can only load one page data! more page consider use with `MetaType::Loop`
#[async_trait]
impl FilterBefore for Loader {
    async fn filter(&self, ins: &mut Instance, cfg: &str) -> Result<()> {
        let setting = match Setting::get(&cfg) {
            Ok(s) => s,
            Err(e) => {
                warn!("loader setting error: {}, \nsetting is: {}", e, cfg);
                return Err(e);
            }
        };
        let time_range = match &setting.time_part {
            Some(part) => match get_para_part(&ins.para, part) {
                Ok(pair) => (
                    Some(i64::from_str(&pair[0])? * 1000),
                    Some(i64::from_str(&pair[1])? * 1000)
                ),
                Err(e) => {
                    let msg = format!("built-in::Loader instance's para has no time info: {}", e.to_string());
                    return Err(NatureError::VerifyError(msg));
                }
            },
            None => (None, None)
        };
        let first = match ins.sys_context.get(CONTEXT_LOOP_NEXT) {
            Some(first) => first.to_string(),
            None => setting.key_gt,
        };
        debug!("loader for: {}, condition first: {}", ins.meta, first);

        // init loop_id
        let loop_id = match ins.sys_context.get(CONTEXT_LOOP_ID) {
            Some(id) => u32::from_str(id)? + 1,
            None => 1
        };
        ins.sys_context.insert(CONTEXT_LOOP_ID.to_string(), loop_id.to_string());

        // load
        let condition = KeyCondition {
            id: 0,
            meta: "".to_string(),
            key_gt: first,
            key_ge: "".to_string(),
            key_lt: setting.key_lt,
            key_le: "".to_string(),
            para: "".to_string(),
            state_version: 0,
            time_ge: time_range.0,
            time_lt: time_range.1,
            limit: setting.page_size as i32,
        };
        let mut content: Vec<String> = vec![];
        let rtn: Vec<Instance> = self.dao.get_by_key_range(&condition).await?;
        let len = rtn.len();
        debug!("loaded records: {} for: {} ", len, ins.meta);

        // set context
        if len == setting.page_size as usize {
            let next = rtn[len - 1].get_key();
            ins.sys_context.insert(CONTEXT_LOOP_NEXT.to_string(), next.to_string());
            debug!("for meta: {} set the next loop from: {}", ins.meta, next);
        } else {
            ins.sys_context.insert(CONTEXT_LOOP_FINISHED.to_string(), "".to_string());
            debug!("finished loop for: {}", ins.meta);
        }
        // filter
        for mut one in rtn {
            convert_before(&mut one, setting.filters.clone()).await?;
            content.push(one.content.to_string());
        }
        ins.content = serde_json::to_string(&content)?;
        debug!("loaded content for: {} is: {}", ins.meta, ins.content);
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
    /// where to get the time range from the `Instance.para` which used to load data from Instance table
    /// it only accept two element, one for Begin Time and the other for End Time
    time_part: Option<Vec<u8>>,
    /// correct the format of the data loaded.
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    filters: Vec<Executor>,
}

impl Setting {
    pub fn get(cfg: &str) -> Result<Setting> {
        if cfg.is_empty() {
            return Err(NatureError::VerifyError("builtin-filter loader `settings` can't be empty".to_string()));
        };
        let result: Setting = serde_json::from_str(cfg)?;
        if let Some(part) = &result.time_part {
            if part.len() != 2 {
                return Err(NatureError::VerifyError("builtin-filter loader `settings.time_part` need exactly 2 elements".to_string()));
            }
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
    fn time_part_not_set() {
        let s = r#"{"key_gt":"abc","key_lt":"def"}"#;
        let ok = Setting::get(s);
        assert_eq!(ok.is_ok(), true);
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

#[cfg(test)]
mod test_ignore {
    use crate::db::RelationSettings;

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct ParaAsKeyTest {
        /// if false add "" around the content.
        #[serde(skip_serializing_if = "is_default")]
        #[serde(default)]
        plain: bool,
        /// where to get the part from the `Instance.para` which used to form a key for content
        part: Vec<u8>,
    }

    #[test]
    #[ignore]
    fn complex_config_test() {
        // filter
        let mut para_as_key = Executor::default();
        let para_test = ParaAsKeyTest { plain: true, part: vec![2] };
        para_as_key.settings = serde_json::to_string(&para_test).unwrap();

        // loader
        let mut loader = Executor::default();
        let loader_setting = Setting {
            key_gt: "".to_string(),
            key_lt: "".to_string(),
            page_size: 0,
            time_part: None,
            filters: vec![para_as_key],
        };
        loader.settings = serde_json::to_string(&loader_setting).unwrap();

        // relation
        let mut relation_settings = RelationSettings::default();
        relation_settings.convert_before = vec![loader];
        let result = serde_json::to_string(&relation_settings).unwrap();
        dbg!(result);
    }
}