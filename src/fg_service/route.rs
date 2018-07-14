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
    fn filter_relations(instance: &Instance, maps: Vec<OneStepFlow>) -> Option<Vec<Target>> {
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

    fn context_check(contexts: &HashMap<String, String>, mapping: &OneStepFlow) -> bool {
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

    fn status_check(status: &HashSet<String>, mapping: &OneStepFlow) -> bool {
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

