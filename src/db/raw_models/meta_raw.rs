use std::convert::TryInto;

use chrono::prelude::*;
use mysql_async::{params, Row, Value};

use crate::domain::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawMeta {
    pub meta_type: String,
    pub meta_key: String,
    /// For human readable what the `Meta` is.
    pub description: Option<String>,
    /// version of the `Meta`
    pub version: i32,
    pub states: Option<String>,
    /// Define whats the `Meta` should include
    pub fields: Option<String>,
    pub config: String,
    pub flag: i32,
    pub create_time: NaiveDateTime,
}

impl Default for RawMeta {
    fn default() -> Self {
        RawMeta {
            meta_type: MetaType::default().get_prefix(),
            meta_key: String::new(),
            description: None,
            version: 1,
            states: None,
            fields: None,
            config: "{}".to_string(),
            flag: 1,
            create_time: Local::now().naive_local(),
        }
    }
}

impl From<Meta> for RawMeta {
    fn from(m: Meta) -> Self {
        RawMeta {
            meta_type: m.get_meta_type().get_prefix(),
            description: None,
            version: m.version as i32,
            states: match m.get_states() {
                None => None,
                Some(x) => Some(State::states_to_string(&x, ","))
            },
            fields: None,
            config: match m.get_setting() {
                None => "".to_string(),
                Some(s) => s.to_json().unwrap()
            },
            flag: 0,
            create_time: Local::now().naive_local(),
            meta_key: m.get_key(),
        }
    }
}

impl TryInto<Meta> for RawMeta {
    type Error = NatureError;

    fn try_into(self) -> std::result::Result<Meta, Self::Error> {
        let mut rtn = Meta::new(&self.meta_key, self.version as u32, MetaType::from_prefix(&self.meta_type)?)?;
        if let Some(s) = &self.states {
            if !s.is_empty() {
                match State::string_to_states(&s) {
                    Ok((ss, _)) => rtn.set_states(Some(ss))?,
                    Err(e) => {
                        warn!("meta : {}:{}:{} init error: {:?}", &self.meta_type, &self.meta_key, self.version, e);
                        return Err(e);
                    }
                }
            }
        }
        let _ = rtn.set_setting(&self.config)?;
        debug!("get meta:{}", rtn.meta_string());
        Ok(rtn)
    }
}

impl From<Row> for RawMeta {
    fn from(row: Row) -> Self {
        let (meta_type, meta_key, description, version, states, fields, config, flag, create_time) = mysql_async::from_row(row);
        RawMeta {
            meta_type,
            meta_key,
            description,
            version,
            states,
            fields,
            config,
            flag,
            create_time,
        }
    }
}

impl Into<Vec<(String, Value)>> for RawMeta {
    fn into(self) -> Vec<(String, Value)> {
        params! {
            "meta_type" => self.meta_type,
            "meta_key" => self.meta_key,
            "description" => self.description,
            "version" => self.version,
            "states" => self.states,
            "fields" => self.fields,
            "config" => self.config,
            "flag" => self.flag,
            "create_time" => self.create_time,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_into_test() {
        // error full_key
        let meta = RawMeta::default();
        let result: Result<Meta> = meta.try_into();
        assert_eq!(result.err().unwrap(), NatureError::VerifyError("key length can't be zero".to_string()));

        let meta = RawMeta::from(Meta::from_string("B:hello:1").unwrap());
        let result: Meta = meta.try_into().unwrap();
        assert_eq!(result.meta_string(), "B:hello:1")
    }

    #[test]
    fn try_into_state_check_test() {
        let mut meta = RawMeta::from(Meta::from_string("B:hello:1").unwrap());
        meta.states = Some("a,b".to_string());
        let result: Result<Meta> = meta.try_into();
        assert_eq!(result.is_ok(), true);

        let mut meta = RawMeta::from(Meta::from_string("B:hello:1").unwrap());
        meta.states = Some("b,b".to_string());
        let result: Result<Meta> = meta.try_into();
        assert_eq!(result.err().unwrap(), NatureError::VerifyError("repeated state name: [b]".to_string()));

        let mut meta = RawMeta::from(Meta::from_string("B:hello:1").unwrap());
        meta.states = Some("".to_string());
        let result: Result<Meta> = meta.try_into();
        assert_eq!(result.is_ok(), true);
    }
}

