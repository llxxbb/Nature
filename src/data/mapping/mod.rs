use data::*;
use std::collections::HashSet;

/// the compose of `Mapping::from`, `Mapping::to` and `Weight::label` must be unique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub from: Thing,
    pub to: Thing,
    pub how: Converter,
    pub demand: Demand,
    pub weight: Weight,
}


/// used to gray deploy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weight {
    /// Used to distinguish converters which are same `Mapping::from` and `Mapping::to`
    pub label: String,
    /// indicate the proportion of the whole stream, the whole will the sum of the participate `Weight::proportion`
    pub proportion: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Converter {}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demand {
    status_include: HashSet<String>,
    status_exclude: HashSet<String>,
    context_include: HashSet<String>,
    context_exclude: HashSet<String>,
}