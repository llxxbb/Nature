use super::*;

pub struct MappingDaoService;

impl MappingDao for MappingDaoService {
    /// The relations will be cached
    fn get_relations(instance: &Instance) -> Result<Option<RouteInfo>> {
        // TODO
        let ins = &*(instance.clone());

        let _router_info = RouteInfo{
            instance: *ins,
            maps: Vec::new(),
        };
        unimplemented!()
    }
}