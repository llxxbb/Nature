use std::rc::Rc;

use super::*;

pub trait RouteServiceTrait {
    fn get_dynamic_mission(&self, dynamic: Vec<DynamicConverter>) -> Result<Vec<Mission>>;
}

pub struct RouteServiceImpl {
    pub task_service: Rc<TaskServiceTrait>,
}

impl RouteServiceTrait for RouteServiceImpl {
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
        debug!("missions : {:?}", missions);
        Ok(missions)
    }
}

#[cfg(test)]
mod selector_test {
    // TODO
//    use super::*;
//
//    #[test]
//    fn source_status_needed() {
//        let mut set = HashSet::<String>::new();
//        set.insert("one".to_string());
//        set.insert("two".to_string());
//
//        let mut instance = Instance::default();
//
//        // set status required.
//        let osf = vec![OneStepFlow::new_for_source_status_needed("from", "to", &set).unwrap()];
//
//        // condition does not satisfy.
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//        instance.data.status = HashSet::new();
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//        instance.data.status.insert("three".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//        instance.data.status.insert("one".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//
//        // condition satisfy
//        instance.data.status.insert("two".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_some(), true);
//        instance.data.status.insert("four".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_some(), true);
//    }
//
//    #[test]
//    fn source_status_exclude() {
//        let mut set = HashSet::<String>::new();
//        set.insert("one".to_string());
//        set.insert("two".to_string());
//
//        let mut instance = Instance::default();
//
//        // set status required.
//        let osf = vec![OneStepFlow::new_for_source_status_excluded("from", "to", &set).unwrap()];
//
//        // condition satisfy
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_some(), true);
//        instance.data.status = HashSet::new();
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_some(), true);
//        instance.data.status.insert("three".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_some(), true);
//
//        // condition does not satisfy
//        instance.data.status.insert("one".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//        instance.data.status.insert("two".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//        instance.data.status.remove("one");
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//    }
//
//    #[test]
//    fn context_needed() {
//        let mut set = HashSet::<String>::new();
//        set.insert("one".to_string());
//        set.insert("two".to_string());
//
//        let mut instance = Instance::default();
//
//        // set status required.
//        let osf = vec![OneStepFlow::new_for_context_include("from", "to", &set).unwrap()];
//
//        // condition does not satisfy.
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//        instance.data.context = HashMap::new();
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//        instance.data.context.insert("three".to_string(), "three".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//        instance.data.context.insert("one".to_string(), "one".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//
//        // condition satisfy
//        instance.data.context.insert("two".to_string(), "two".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_some(), true);
//        instance.data.context.insert("four".to_string(), "four".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_some(), true);
//    }
//
//    #[test]
//    fn context_exclude_test() {
//        let mut set = HashSet::<String>::new();
//        set.insert("one".to_string());
//        set.insert("two".to_string());
//
//        let mut instance = Instance::default();
//
//        // set status required.
//        let osf = vec![OneStepFlow::new_for_context_excluded("from", "to", &set).unwrap()];
//
//        // condition satisfy
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_some(), true);
//        instance.data.context = HashMap::new();
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_some(), true);
//        instance.data.context.insert("three".to_string(), "three".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_some(), true);
//
//        // condition does not satisfy
//        instance.data.context.insert("one".to_string(), "one".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//        instance.data.context.insert("two".to_string(), "two".to_string());
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//        instance.data.context.remove("one");
//        let option = RouteServiceImpl::filter_relations(&instance, osf.clone());
//        assert_eq!(option.is_none(), true);
//    }
}

#[cfg(test)]
mod other_test {
    // TODO
//    use super::*;
//
//    #[test]
//    fn input_cfg_is_empty() {
//        let instance = Instance::default();
//        let osf: Vec<OneStepFlow> = Vec::new();
//        let option = Mission::filter_relations(&instance, osf);
//        assert_eq!(option.is_none(), true)
//    }
//
//    #[test]
//    fn no_selector_but_only_executor() {
//        let instance = Instance::default();
//        let osf = vec![OneStepFlow::new_for_local_executor("from", "to", "local").unwrap()];
//        let option = Mission::filter_relations(&instance, osf);
//        assert_eq!(option.unwrap().len(), 1)
//    }
}


