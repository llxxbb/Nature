use mysql_async::{params, Value};
use mysql_async::Row;
use serde_json;

use crate::common::*;
use crate::db::RelationSettings;

#[derive(Debug)]
#[derive(Clone)]
pub struct RawRelation {
    pub from_meta: String,
    pub to_meta: String,
    pub settings: String,
    pub flag: i32,
}

impl RawRelation {
    pub fn new(from: &str, to: &str, settings: &RelationSettings) -> Result<Self> {
        let st = serde_json::to_string(settings)?;
        let rtn = RawRelation {
            from_meta: from.to_string(),
            to_meta: to.to_string(),
            settings: st,
            flag: 1,
        };
        Ok(rtn)
    }

    pub fn get_string(&self) -> String {
        format!("relation[{}  --->  {}]", self.from_meta, self.to_meta)
    }
}

impl From<Row> for RawRelation {
    fn from(row: Row) -> Self {
        let (from_meta, to_meta, settings, flag) = mysql_async::from_row(row);
        RawRelation {
            from_meta,
            to_meta,
            settings,
            flag,
        }
    }
}

impl Into<Vec<(String, Value)>> for RawRelation {
    fn into(self) -> Vec<(String, Value)> {
        params! {
            "from_meta" => self.from_meta,
            "to_meta" => self.to_meta,
            "settings" => self.settings,
            "flag" => self.flag,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_string_test() {
        let result = RawRelation::new("a", "b", &RelationSettings::default()).unwrap();
        assert_eq!(result.get_string(), "relation[a  --->  b]")
    }
}

