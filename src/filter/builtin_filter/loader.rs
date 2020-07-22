use std::str::FromStr;
use std::sync::Arc;

use nature_common::{Executor, get_para_part, Instance, KeyCondition, NatureError, Result};
use nature_db::KeyGT;

use crate::filter::builtin_filter::FilterBefore;

pub struct Loader {
    pub dao: Arc<dyn KeyGT>
}

#[async_trait]
impl FilterBefore for Loader {
    async fn filter(&self, ins: &mut Instance, cfg: &str) -> Result<()> {
        let setting = Setting::get(&cfg)?;
        let time_range = get_para_part(&ins.para, &setting.time_part)?;
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
        loop {
            let rtn: Vec<Instance> = self.dao.get_by_key_gt(&condition).await?;
            let len = rtn.len();
            if len == setting.page_size as usize {
                condition.key_gt = rtn[len - 1].get_key();
            }
            for one in rtn {
                // TODO embedded filter

                content.push(one.content.to_string());
            }
            if len < setting.page_size as usize {
                break;
            }
        }
        // change the content
        Ok(())
    }
}


/// when used this mode the target `MetaType` must be `Multi`
#[derive(Serialize, Deserialize)]
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
