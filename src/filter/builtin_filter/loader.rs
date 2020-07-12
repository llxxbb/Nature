use nature_common::{Executor, Instance, KeyCondition, NatureError, Result};

pub fn loader(_para: &mut Instance, cfg: &str) -> Result<()> {
    let setting = Setting::get(cfg)?;
    let condition = KeyCondition {
        id: "".to_string(),
        meta: "".to_string(),
        key_gt: setting.key_like,
        para: "".to_string(),
        state_version: 0,
        time_ge: None,
        time_lt: None,
        limit: 0,
    };

    Ok(())
}


/// when used this mode the target `MetaType` must be `Multi`
#[derive(Serialize, Deserialize)]
struct Setting {
    /// the prefix of `ins_key`
    key_like: String,
    #[serde(skip_serializing_if = "is_100")]
    #[serde(default = "default_100")]
    page_size: u16,
    /// where to get the time range from the `Instance'para` which used to load data from Instance table
    time_part: Vec<u8>,
    /// correct the format of the data loaded.
    /// the needed target data format is : [key],[value1],[value2],[value3],...
    /// for example: item1,2,100  // the custom bought 2 item1 and paid $100.
    filters: Vec<Executor>,
}

impl Setting {
    pub fn get(cfg: &str) -> Result<Setting> {
        if cfg.is_empty() {
            return Err(NatureError::VerifyError("builtin-filter loader `settings` can't be empty".to_string()));
        };
        let result = serde_json::from_str(cfg)?;
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
