use mysql_async::{params, Params};
use mysql_async::Row;
use serde_json;

use crate::db::RelationSettings;
use crate::domain::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRelation {
    pub id: i32,
    pub from_meta: String,
    pub to_meta: String,
    pub settings: String,
    pub flag: i32,
}

impl RawRelation {
    pub fn new(from: &str, to: &str, settings: &RelationSettings) -> Result<Self> {
        let st = serde_json::to_string(settings)?;
        let rtn = RawRelation {
            id: 0,
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
        let (id, from_meta, to_meta, settings, flag) = mysql_async::from_row(row);
        RawRelation {
            id,
            from_meta,
            to_meta,
            settings,
            flag,
        }
    }
}

impl Into<Params> for RawRelation {
    fn into(self) -> Params {
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

