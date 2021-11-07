use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use chrono::prelude::*;
use lazy_static::__Deref;
use mysql_async::{params, Row, Value};
use serde_json;

use crate::domain::*;
use crate::util::*;

pub struct RawInstance {
    meta: String,
    ins_id: u64,
    para: String,
    content: String,
    context: Option<String>,
    states: Option<String>,
    state_version: i32,
    create_time: NaiveDateTime,
    sys_context: Option<String>,
    from_key: String,
}

impl RawInstance {
    pub fn to(&self) -> Result<Instance> {
        let from = if self.from_key.eq("") { None } else {
            Some(InstanceLocator::from_str(&self.from_key)?)
        };
        let context = match self.context {
            None => HashMap::new(),
            Some(ref s) => serde_json::from_str::<HashMap<String, String>>(s)?
        };
        let sys_context = match self.sys_context {
            None => HashMap::new(),
            Some(ref s) => serde_json::from_str::<HashMap<String, String>>(s)?
        };
        let states = match self.states {
            None => HashSet::new(),
            Some(ref s) => serde_json::from_str::<HashSet<String>>(s)?
        };
        let time = match Local.from_local_datetime(&self.create_time).single() {
            Some(t) => t,
            None => {
                let msg = format!("instance create time error: {}|{}|{}", self.meta, self.ins_id, self.para);
                error!("{}", msg);
                return Err(NatureError::VerifyError(msg));
            }
        };
        Ok(Instance {
            id: self.ins_id,
            path: Modifier {
                meta: self.meta.clone(),
                state_version: self.state_version,
                para: self.para.clone(),
            },
            data: BizObject {
                content: self.content.clone(),
                context,
                sys_context,
                states,
            },
            from,
            create_time: time.timestamp_millis(),
        })
    }

    pub fn new(instance: &Instance) -> Result<RawInstance> {
        Ok(RawInstance {
            meta: instance.path.meta.to_string(),
            ins_id: instance.id,
            para: instance.path.para.to_string(),
            content: {
                if instance.content.len() > *INSTANCE_CONTENT_MAX_LENGTH.deref() {
                    return Err(NatureError::SystemError("content's length can' be over : ".to_owned() + &INSTANCE_CONTENT_MAX_LENGTH.to_string()));
                }
                instance.content.clone()
            },
            context: Self::context_to_raw(&instance.context, "context")?,
            states: match instance.states.len() {
                0 => None,
                _ => Some(serde_json::to_string(&instance.states)?)
            },
            state_version: instance.path.state_version,
            create_time: Local.timestamp_millis(instance.create_time).naive_local(),
            sys_context: Self::context_to_raw(&instance.sys_context, "sys_context")?,
            from_key: match &instance.from {
                None => "".to_string(),
                Some(from) => from.to_string()
            },
        })
    }

    fn context_to_raw(context: &HashMap<String, String>, which: &str) -> Result<Option<String>> {
        let ctx_len = context.len();
        if ctx_len > *INSTANCE_CONTEXT_MAX_LENGTH.deref() {
            let msg = format!("{}'s length can' be over : {}", which, INSTANCE_CONTEXT_MAX_LENGTH.to_string());
            return Err(NatureError::SystemError(msg));
        }
        match ctx_len {
            0 => Ok(None),
            _ => Ok(Some(serde_json::to_string(context)?))
        }
    }
}

impl From<Row> for RawInstance {
    fn from(row: Row) -> Self {
        let (meta, ins_id, para, content, context, states, state_version, create_time, sys_context, from_key) = mysql_async::from_row(row);
        RawInstance {
            meta,
            ins_id,
            para,
            content,
            context,
            states,
            state_version,
            create_time,
            sys_context,
            from_key,
        }
    }
}

impl Into<Vec<(String, Value)>> for RawInstance {
    fn into(self) -> Vec<(String, Value)> {
        params! {
            "meta" => self.meta,
            "ins_id" => self.ins_id,
            "para" => self.para,
            "content" => self.content,
            "context" => self.context,
            "states" => self.states,
            "state_version" => self.state_version,
            "create_time" => self.create_time,
            "sys_context" => self.sys_context,
            "from_key" => self.from_key,
        }
    }
}
