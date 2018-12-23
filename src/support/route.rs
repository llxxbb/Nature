use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use super::*;

pub trait RouteServiceTrait {
    fn get_route(&self, instance: &Instance) -> Result<Option<Vec<Mission>>>;
}

pub struct RouteServiceImpl {
    pub task_service: Rc<TaskServiceTrait>,
    pub one_step_flow_cache: Rc<OneStepFlowCacheTrait>,
}

impl RouteServiceTrait for RouteServiceImpl {
    fn get_route(&self, instance: &Instance) -> Result<Option<Vec<Mission>>> {
        debug!("------------------get_route------------------------");
        let key = &instance.thing.key;
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
}

impl RouteServiceImpl {
    fn filter_relations(instance: &Instance, maps: Vec<OneStepFlow>) -> Option<Vec<Mission>> {
        debug!("------------------filter_relations------------------------");
        let mut rtn: Vec<Mission> = Vec::new();
        for m in maps {
            if m.selector.is_some() {
                let selector = &m.selector.clone().unwrap();
                if !Self::context_check(&instance.data.context, selector) {
                    //        debug!("filter relations for instance: {}", &instance.thing.key);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_cfg_is_empty() {
        let instance = Instance::default();
        let osf : Vec<OneStepFlow> = Vec::new();
        let option = RouteServiceImpl::filter_relations(&instance, osf);
        assert_eq!(option.is_none(), true)
    }

    #[test]
    fn instance_context_is_need_but_not_set() {
        let instance = Instance::default();
        let osf = vec![OneStepFlow{
            from: Thing::new("from")?,
            to: Thing::new("to")?,
            selector: Some(Selector{
                source_status_include: HashSet::new(),
                source_status_exclude: HashSet::new(),
                target_status_include: HashSet::new(),
                target_status_exclude: HashSet::new(),
                context_include: HashSet::new(),
                context_exclude: HashSet::new()
            }),
            executor: Executor::default()
        }];

        let option = RouteServiceImpl::filter_relations(&instance, osf);
        assert_eq!(option.is_none(), true)
    }

    #[test]
    fn group_test() {}

    #[test]
    fn weight_test() {}
}

