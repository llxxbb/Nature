use std::str::FromStr;
use crate::common::*;

use crate::domain::*;
use crate::util::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ConverterReturned {
    /// This will break process for ever.
    LogicalError { msg: String },
    /// This can quick finish the process, and retry later.
    EnvError { msg: String },
    /// No instance would be return.
    None,
    /// Tell `Nature` the task will be processed asynchronously, Nature will wait for seconds you assigned, and converter will callback to `Nature` later while result are ready.
    Delay { num: u32 },
    /// return instances
    Instances { ins: Vec<Instance> },
    /// return `SelfRouteInstance`
    SelfRoute { ins: Vec<SelfRouteInstance> },
}

impl Default for ConverterReturned {
    fn default() -> Self {
        ConverterReturned::None
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConverterParameter {
    pub from: Instance,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub last_state: Option<Instance>,
    /// This is used for callback
    pub task_id: u64,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub master: Option<Instance>,
    /// executor setting
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub cfg: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct DynamicConverter {
    /// Only `Dynamic` and `Null` metaType support
    pub to: Option<String>,
    /// REST api for convert to `to`
    pub fun: Executor,
    /// use upstream's id as downstream's id.
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub use_upstream_id: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub delay: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Protocol {
    LocalRust,
    Http,
    Https,
    /// Nature will automatically implement the converter. it can't be used by user.
    Auto,
    BuiltIn,
}

impl FromStr for Protocol {
    type Err = NatureError;

    fn from_str(s: &str) -> Result<Self> {
        let cmp = &*s.to_uppercase();
        match cmp {
            "LOCALRUST" => Ok(Protocol::LocalRust),
            "HTTP" => Ok(Protocol::Http),
            "HTTPS" => Ok(Protocol::Https),
            "BUILTIN" => Ok(Protocol::BuiltIn),
            _ => {
                let msg = format!("unknown protocol : {}", s);
                Err(NatureError::VerifyError(msg))
            }
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::LocalRust
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct Executor {
    pub protocol: Protocol,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub url: String,
    /// A json string which resolved by executor itself
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub settings: String,
}

impl Executor {
    pub fn for_local(path: &str) -> Self {
        Executor {
            protocol: Protocol::LocalRust,
            url: path.to_string(),
            settings: "".to_string(),
        }
    }

    pub fn new_auto() -> Self {
        Executor {
            protocol: Protocol::Auto,
            url: "".to_string(),
            settings: "".to_string(),
        }
    }
}

#[cfg(test)]
mod executor_test {
    use super::*;

    #[test]
    fn serde_executor() {
        let exe = Executor {
            protocol: Protocol::LocalRust,
            url: "".to_string(),
            settings: "".to_string(),
        };
        let ewe_s = serde_json::to_string(&exe).unwrap();
        assert_eq!(ewe_s, "{\"protocol\":\"localRust\"}");
        let ewe_dw: Executor = serde_json::from_str(&ewe_s).unwrap();
        assert_eq!(ewe_dw, exe);
    }
}

#[cfg(test)]
mod converter_returned_test {
    use super::*;

    #[test]
    fn none_test() {
        // converterReturned
        let none = ConverterReturned::None;
        let result = serde_json::to_string(&none).unwrap();
        println!("{}", &result);
        let back: ConverterReturned = serde_json::from_str(&result).unwrap();
        assert_eq!(none, back)
    }

    #[test]
    fn logical_error_test() {
        // converterReturned
        let original = ConverterReturned::LogicalError { msg: "some error".to_string() };
        let result = serde_json::to_string(&original).unwrap();
        println!("{}", &result);
        let back: ConverterReturned = serde_json::from_str(&result).unwrap();
        assert_eq!(original, back)
    }

    #[test]
    fn env_error_test() {
        // converterReturned
        let original = ConverterReturned::EnvError { msg: "some error".to_string() };
        let result = serde_json::to_string(&original).unwrap();
        println!("{}", &result);
        let back: ConverterReturned = serde_json::from_str(&result).unwrap();
        assert_eq!(original, back)
    }

    #[test]
    fn delay_test() {
        // converterReturned
        let original = ConverterReturned::Delay { num: 10 };
        let result = serde_json::to_string(&original).unwrap();
        println!("{}", &result);
        let back: ConverterReturned = serde_json::from_str(&result).unwrap();
        assert_eq!(original, back)
    }

    #[test]
    fn instance_test() {
        // converterReturned
        let original = ConverterReturned::Instances { ins: vec![Instance::default()] };
        let result = serde_json::to_string(&original).unwrap();
        println!("{}", &result);
        let back: ConverterReturned = serde_json::from_str(&result).unwrap();
        assert_eq!(original, back)
    }

    #[test]
    fn self_route_test() {
        // converterReturned
        let original = ConverterReturned::SelfRoute { ins: vec![SelfRouteInstance::default()] };
        let result = serde_json::to_string(&original).unwrap();
        println!("{}", &result);
        let back: ConverterReturned = serde_json::from_str(&result).unwrap();
        assert_eq!(original, back)
    }
}
