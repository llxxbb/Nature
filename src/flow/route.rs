use global::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;
use super::*;

pub trait RouteServiceTrait {
    fn get_route(instance: &Instance) -> Result<Option<Vec<Mission>>>;
}

pub struct RouteServiceImpl<D, O> {
    delivery_service: PhantomData<D>,
    one_step_flow_cache: PhantomData<O>,
}

impl<D, O> RouteServiceTrait for RouteServiceImpl<D, O>
    where D: DeliveryServiceTrait, O: OneStepFlowCacheTrait {
    fn get_route(instance: &Instance) -> Result<Option<Vec<Mission>>> {
        if let Ok(Some(relations)) = O::get(&instance.thing) {
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
    fn filter_relations(instance: &Instance, maps: Vec<OneStepFlow>) -> Option<Vec<Mission>> {
//        debug!("filter relations for instance: {:?}", instance);
        let mut rtn: Vec<Mission> = Vec::new();
        for m in maps {
            if !m.selector.is_none() {
                let selector = &m.selector.clone().unwrap();
                if !Self::context_check(&instance.data.context, selector) {
                    continue;
                }
                if !Self::status_check(&instance.data.status, selector) {
                    continue;
                }
            }
            let t = Mission {
                to: m.to.clone(),
                executor: m.executor,
                last_status_demand: {
                    match m.selector {
                        None => None,
                        Some(demand) => {
                            let ld = LastStatusDemand {
                                target_status_include: demand.target_status_include,
                                target_status_exclude: demand.target_status_exclude,
                            };
                            Some(ld)
                        }
                    }
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

    fn context_check(contexts: &HashMap<String, String>, selector: &Selector) -> bool {
        for exclude in &selector.context_exclude {
            if contexts.contains_key(exclude) {
                return false;
            }
        }
        for include in &selector.context_include {
            if !contexts.contains_key(include) {
                return false;
            }
        }
        true
    }

    fn status_check(status: &HashSet<String>, selector: &Selector) -> bool {
        for exclude in &selector.source_status_exclude {
            if status.contains(exclude) {
                return false;
            }
        }
        for include in &selector.source_status_include {
            if !status.contains(include) {
                return false;
            }
        }
        true
    }
}

