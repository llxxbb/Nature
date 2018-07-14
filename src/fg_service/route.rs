use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;
use super::*;

pub trait RouteServiceTrait {
    fn get_route(instance: &Instance) -> Result<Option<Vec<Target>>>;
}

pub struct RouteServiceImpl<D, O> {
    delivery_service: PhantomData<D>,
    one_step_flow_cache: PhantomData<O>,
}

impl<D, O> RouteServiceTrait for RouteServiceImpl<D, O>
    where D: DeliveryServiceTrait, O: OneStepFlowCacheTrait {
    fn get_route(instance: &Instance) -> Result<Option<Vec<Target>>> {
        if let Ok(relations) = O::get(&instance.thing) {
            // no relations
            if relations.len() == 0 {
                return Ok(None);
            }
            let rtn = Self::filter_relations(instance, relations);
            Ok(rtn)
        } else {
            Ok(None)
        }
    }
}

impl<D, O> RouteServiceImpl<D, O> {
    fn filter_relations(instance: &Instance, maps: Vec<Relation>) -> Option<Vec<Target>> {
        debug!("filter relations for instance: {:?}", instance);
        let mut rtn: Vec<Target> = Vec::new();
        for m in maps {
            if !Self::context_check(&instance.data.context, &m) {
                continue;
            }
            if !Self::status_check(&instance.data.status, &m) {
                continue;
            }
            let t = Target {
                to: m.to.clone(),
                executor: m.who,
                last_status_demand: LastStatusDemand {
                    target_status_include: m.demand.target_status_include,
                    target_status_exclude: m.demand.target_status_exclude,
                },
                weight: m.weight,
            };
            rtn.push(t);
        }
        match rtn.len() {
            x  if x > 0 => {
                return Some(rtn);
            }
            _ => return None
        }
    }

    fn context_check(contexts: &HashMap<String, String>, mapping: &Relation) -> bool {
        for exclude in &mapping.demand.context_exclude {
            if contexts.contains_key(exclude) {
                return false;
            }
        }
        for include in &mapping.demand.context_include {
            if !contexts.contains_key(include) {
                return false;
            }
        }
        true
    }

    fn status_check(status: &HashSet<String>, mapping: &Relation) -> bool {
        for exclude in &mapping.demand.source_status_exclude {
            if status.contains(exclude) {
                return false;
            }
        }
        for include in &mapping.demand.source_status_include {
            if !status.contains(include) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteInfo {
    pub instance: Instance,
    pub maps: Vec<Relation>,
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
    pub executor: Converter,
    pub last_status_demand: LastStatusDemand,
    pub weight: Weight,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Demand {
    pub source_status_include: HashSet<String>,
    pub source_status_exclude: HashSet<String>,
    pub target_status_include: HashSet<String>,
    pub target_status_exclude: HashSet<String>,
    pub context_include: HashSet<String>,
    pub context_exclude: HashSet<String>,
}


/// the compose of `Mapping::from`, `Mapping::to` and `Weight::label` must be unique
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Relation {
    pub from: Thing,
    pub to: Thing,
    pub who: Converter,
    pub demand: Demand,
    pub weight: Weight,
}

/// used to gray deploy
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Weight {
    /// Used to distinguish converters which are same `Mapping::from` and `Mapping::to`
    pub label: String,
    /// indicate the proportion of the whole stream, the whole will the sum of the participate `Weight::proportion`
    pub proportion: u8,
}


