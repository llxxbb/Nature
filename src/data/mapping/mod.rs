use data::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub from: Thing,
    pub to: Thing,
    pub how: Converter,
    pub option: Demand,
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