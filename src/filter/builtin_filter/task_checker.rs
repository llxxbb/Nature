use std::str::FromStr;

use chrono::{Local, TimeZone};

use nature_common::{get_para_part, Instance, is_default, NatureError, Result};
use nature_db::task_check::{Condition, TaskChecker};

use crate::filter::builtin_filter::FilterBefore;

pub struct TaskCheckerFilter;

#[async_trait]
impl FilterBefore for TaskCheckerFilter {
    async fn filter(&self, ins: &mut Instance, cfg: &str) -> Result<()> {
        // deserialize Setting
        let cfg: Setting = match serde_json::from_str(cfg) {
            Ok(rtn) => rtn,
            Err(e) => {
                let msg = format!("TaskCheckerFilter get error: {}, cfg: {}", e, cfg);
                warn!("{}", msg);
                return Err(NatureError::VerifyError(msg));
            }
        };
        // get query condition
        let condition = cfg.to_condition(ins)?;
        // check
        let num = TaskChecker::check(&condition).await?;
        if num == 0 {
            Ok(())
        } else {
            let msg = format!("tasks are unready for ins: {} to convert", ins.get_key());
            warn!("{}", msg);
            Err(NatureError::EnvironmentError(msg))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Setting {
    /// great than `task_key`
    key_gt: String,
    /// less equal `task_key`
    key_lt: String,
    /// where to get the time range from the `Instance'para` which used to load data from Instance table
    /// it only accept two element, one for Begin Time and the other for End Time
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    time_part: Option<Vec<u8>>,
}

impl Setting {
    fn to_condition(&self, ins: &Instance) -> Result<Condition> {
        // get time info from para
        let part = match &self.time_part {
            Some(part) => match get_para_part(&ins.para, part) {
                Ok(rtn) => {
                    let t_ge = i64::from_str(&rtn[0])? * 1000;
                    let t_lt = i64::from_str(&rtn[1])? * 1000;
                    let t_ge = Local.timestamp_millis(t_ge).naive_local();
                    let t_lt = Local.timestamp_millis(t_lt).naive_local();
                    (Some(t_ge), Some(t_lt))
                }
                Err(e) => {
                    let msg = format!("TaskCheckerFilter: instance's para has no time info: {}， para: {}", e.to_string(), ins.para);
                    return Err(NatureError::VerifyError(msg));
                }
            }
            None => (None, None)
        };

        let rtn = Condition {
            key_gt: self.key_gt.to_string(),
            key_lt: self.key_lt.to_string(),
            time_ge: part.0,
            time_lt: part.1,
            state: 0,
        };
        Ok(rtn)
    }
}