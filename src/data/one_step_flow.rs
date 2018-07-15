use data::instance::Instance;
use data::thing::Thing;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Executor {
    pub protocol: String,
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

/// the compose of `Mapping::from`, `Mapping::to` and `Weight::label` must be unique
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Target {
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



