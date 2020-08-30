use crate::common::{Result, SEPARATOR_META};
use crate::common::error::NatureError;

/// Every `Meta` must have a type
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub enum MetaType {
    Business,
    System,
    Dynamic,
    Null,
    Multi,
    Loop,
}

impl Default for MetaType {
    fn default() -> Self {
        MetaType::Business
    }
}

impl MetaType {
    pub fn get_prefix(&self) -> String {
        match self {
            MetaType::Business => "B".to_string(),
            MetaType::System => "S".to_string(),
            MetaType::Dynamic => "D".to_string(),
            MetaType::Null => "N".to_string(),
            MetaType::Multi => "M".to_string(),
            MetaType::Loop => "L".to_string(),
        }
    }

    pub fn from_prefix(prefix: &str) -> Result<Self> {
        match prefix {
            "B" => Ok(MetaType::Business),
            "S" => Ok(MetaType::System),
            "D" => Ok(MetaType::Dynamic),
            "N" => Ok(MetaType::Null),
            "M" => Ok(MetaType::Multi),
            "L" => Ok(MetaType::Loop),
            _ => Err(NatureError::VerifyError("unknow prefix : [".to_string() + prefix + "]"))
        }
    }

    pub fn check_type(meta: &str, m_type: MetaType) -> Result<()> {
        let prefix = m_type.get_prefix();
        let parts: Vec<&str> = meta.split(&*SEPARATOR_META).collect();
        if parts.len() < 1 {
            let msg = "meta type undefined";
            warn!("{}", msg);
            return Err(NatureError::VerifyError(msg.to_string()));
        }
        let x = parts[0];
        if x != format!("{}", &prefix) {
            let msg = format!("[{}]'s MetaType unwanted", meta);
            return Err(NatureError::VerifyError(msg.to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_profix() {
        assert_eq!("N", MetaType::Null.get_prefix());
        assert_eq!("S", MetaType::System.get_prefix());
        assert_eq!("D", MetaType::Dynamic.get_prefix());
        assert_eq!("B", MetaType::Business.get_prefix());
        assert_eq!("M", MetaType::Multi.get_prefix());
        assert_eq!("L", MetaType::Loop.get_prefix());
    }

    #[test]
    fn from_profix() {
        assert_eq!(MetaType::Null, MetaType::from_prefix("N").unwrap());
        assert_eq!(MetaType::Business, MetaType::from_prefix("B").unwrap());
        assert_eq!(MetaType::System, MetaType::from_prefix("S").unwrap());
        assert_eq!(MetaType::Dynamic, MetaType::from_prefix("D").unwrap());
        assert_eq!(MetaType::Multi, MetaType::from_prefix("M").unwrap());
        assert_eq!(MetaType::Loop, MetaType::from_prefix("L").unwrap());
        assert_eq!(Err(NatureError::VerifyError("unknow prefix : [/d]".to_string())), MetaType::from_prefix("/d"));
    }

    #[test]
    fn check_type_test() {
        assert_eq!(Ok(()), MetaType::check_type("D:dynamic/target/is/null:1", MetaType::Dynamic));
    }
}