use super::*;

pub struct MappingDaoService;

impl MappingDao for MappingDaoService {
    /// The relations will be cached
    fn get_relations(instance: &Instance) -> Result<Option<RouteInfo>> {
        let _router_info = RouteInfo {
            instance: instance.clone(),
            maps: Vec::new(),
        };
        unimplemented!()
    }
}