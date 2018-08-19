use global::*;
use nature_common::*;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Ord, PartialOrd, Eq)]
pub enum Protocol {
    LocalRust,
    Http,
    Https,
}

impl FromStr for Protocol {
    type Err = NatureErrorWrapper;

    fn from_str(s: &str) -> Result<Self> {
        let cmp = &*s.to_uppercase();
        match cmp {
            "LOCALRUST" => Ok(Protocol::LocalRust),
            "HTTP" => Ok(Protocol::Http),
            "HTTPS" => Ok(Protocol::Https),
            _ => {
                let msg = format!("unknown protocol : {}", s);
                Err(NatureErrorWrapper::from(NatureError::VerifyError(msg)))
            }
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::LocalRust
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
pub struct Executor {
    pub protocol: Protocol,
    /// url do not contain's protocol define
    pub url: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RouteInfo {
    pub instance: Instance,
    pub maps: Vec<OneStepFlow>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LastStatusDemand {
    pub target_status_include: HashSet<String>,
    pub target_status_exclude: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConverterInfo {
    pub from: Instance,
    pub target: Mission,
    pub last_status: Option<Instance>,
}

/// the compose of `Mapping::from`, `Mapping::to` and `Weight::label` must be unique
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Mission {
    pub to: Thing,
    pub executor: Executor,
    pub last_status_demand: Option<LastStatusDemand>,
    pub weight: Option<Weight>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Selector {
    pub source_status_include: HashSet<String>,
    pub source_status_exclude: HashSet<String>,
    pub target_status_include: HashSet<String>,
    pub target_status_exclude: HashSet<String>,
    pub context_include: HashSet<String>,
    pub context_exclude: HashSet<String>,
}


/// the compose of `Mapping::from`, `Mapping::to` and `Weight::label` must be unique
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OneStepFlow {
    pub from: Thing,
    pub to: Thing,
    pub executor: Executor,
    pub selector: Option<Selector>,
    pub weight: Option<Weight>,
}

/// used to gray deploy
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Weight {
    /// Used to distinguish converters which are same `OneStepFlow::from` and `OneStepFlow::to`
    pub label: String,
    /// indicate the proportion of the whole stream, the whole will the sum of the participate `Weight::proportion`
    pub proportion: i32,
}



