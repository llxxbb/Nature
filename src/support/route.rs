use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use super::*;

pub trait RouteServiceTrait {
    fn get_mission(&self, instance: &Instance) -> Result<Option<Vec<Mission>>>;
    fn get_dynamic_mission(&self, dynamic: Vec<DynamicConverter>) -> Result<Vec<Mission>>;
}

pub struct RouteServiceImpl {
    pub task_service: Rc<TaskServiceTrait>,
    pub one_step_flow_cache: Rc<OneStepFlowCacheTrait>,
}

impl RouteServiceTrait for RouteServiceImpl {
    fn get_mission(&self, instance: &Instance) -> Result<Option<Vec<Mission>>> {
        debug!("------------------get_route------------------------");
        let key = &instance.thing.get_full_key();
        match self.one_step_flow_cache.get(&instance.thing) {
            Ok(rtn) => {
                match rtn {
                    Some(relations) => {
                        let rtn = Self::filter_relations(instance, relations);
                        Ok(rtn)
                    }
                    None => Ok(None)
                }
            }
            Err(e) => {
                debug!("occur error when routing for biz:err {}:{}", key, e);
                Err(e)
            }
        }
    }
    fn get_dynamic_mission(&self, dynamic: Vec<DynamicConverter>) -> Result<Vec<Mission>> {
        debug!("------------------get_dynamic_route------------------------");
        let mut missions: Vec<Mission> = Vec::new();
        for d in dynamic {
            let t = match d.to {
                None => Thing::new_null(),
                Some(s) => Thing::new_with_type(&s, ThingType::Dynamic)?,
            };
            let mission = Mission {
                to: t,
                executor: d.fun.clone(),
                last_status_demand: None,
            };
            missions.push(mission)
        }
        Ok(missions)
    }
}

impl RouteServiceImpl {
    fn filter_relations(instance: &Instance, maps: Vec<OneStepFlow>) -> Option<Vec<Mission>> {
        debug!("------------------filter_relations------------------------");
        let mut rtn: Vec<Mission> = Vec::new();
        for m in maps {
            if m.selector.is_some() {
                let selector = &m.selector.clone().unwrap();
                if !Self::context_check(&instance.data.context, selector) {
                    continue;
                }
                // only verify source status, target status will be checked later.
                if !Self::source_status_check(&instance.data.status, selector) {
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
                            let last_demand = LastStatusDemand {
                                target_status_include: demand.target_status_include,
                                target_status_exclude: demand.target_status_exclude,
                            };
                            Some(last_demand)
                        }
                    }
                },
            };
            rtn.push(t);
        }
        match rtn.len() {
            x  if x > 0 => {
                Some(rtn)
            }
            _ => None
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

    fn source_status_check(status: &HashSet<String>, selector: &Selector) -> bool {
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

#[cfg(test)]
mod selector_test {
    use super::*;

    #[test]
    fn source_status_needed() {
        let mut set = HashSet::<String>::new();
        set.insert("one".to_string());
        set.insert("two".to_string());

        let mut instance = Instance::default();

        // set status required.
        let osf = vec![OneStepFlow::new_for_source_status_needed("from", "to", &set).unwrap()];

        // condition does not satisfy.
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
        instance.data.status = HashSet::new();
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
        instance.data.status.insert("three".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
        instance.data.status.insert("one".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);

        // condition satisfy
        instance.data.status.insert("two".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_some(), true);
        instance.data.status.insert("four".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_some(), true);
    }

    #[test]
    fn source_status_exclude() {
        let mut set = HashSet::<String>::new();
        set.insert("one".to_string());
        set.insert("two".to_string());

        let mut instance = Instance::default();

        // set status required.
        let osf = vec![OneStepFlow::new_for_source_status_excluded("from", "to", &set).unwrap()];

        // condition satisfy
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_some(), true);
        instance.data.status = HashSet::new();
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_some(), true);
        instance.data.status.insert("three".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_some(), true);

        // condition does not satisfy
        instance.data.status.insert("one".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
        instance.data.status.insert("two".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
        instance.data.status.remove("one");
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
    }

    #[test]
    fn context_needed() {
        let mut set = HashSet::<String>::new();
        set.insert("one".to_string());
        set.insert("two".to_string());

        let mut instance = Instance::default();

        // set status required.
        let osf = vec![OneStepFlow::new_for_context_include("from", "to", &set).unwrap()];

        // condition does not satisfy.
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
        instance.data.context = HashMap::new();
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
        instance.data.context.insert("three".to_string(), "three".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
        instance.data.context.insert("one".to_string(), "one".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);

        // condition satisfy
        instance.data.context.insert("two".to_string(), "two".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_some(), true);
        instance.data.context.insert("four".to_string(), "four".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_some(), true);
    }

    #[test]
    fn context_exclude_test() {
        let mut set = HashSet::<String>::new();
        set.insert("one".to_string());
        set.insert("two".to_string());

        let mut instance = Instance::default();

        // set status required.
        let osf = vec![OneStepFlow::new_for_context_excluded("from", "to", &set).unwrap()];

        // condition satisfy
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_some(), true);
        instance.data.context = HashMap::new();
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_some(), true);
        instance.data.context.insert("three".to_string(), "three".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_some(), true);

        // condition does not satisfy
        instance.data.context.insert("one".to_string(), "one".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
        instance.data.context.insert("two".to_string(), "two".to_string());
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
        instance.data.context.remove("one");
        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
        assert_eq!(option.is_none(), true);
    }
}

#[cfg(test)]
mod other_test {
    use super::*;

    #[test]
    fn input_cfg_is_empty() {
        let instance = Instance::default();
        let osf: Vec<OneStepFlow> = Vec::new();
        let option = RouteServiceImpl::filter_relations(&instance, osf);
        assert_eq!(option.is_none(), true)
    }

    #[test]
    fn no_selector_but_only_executor() {
        let instance = Instance::default();
        let osf = vec![OneStepFlow::new_for_local_executor("from", "to", "local").unwrap()];
        let option = RouteServiceImpl::filter_relations(&instance, osf);
        assert_eq!(option.unwrap().len(), 1)
    }
}


