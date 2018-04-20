use super::*;

pub struct MappingDaoService;

impl MappingDao for MappingDaoService {
    /// The relations will be cached
    fn get_relations(_instance: &Instance) -> Result<Option<RouteInfo>> {
        // TODO
        unimplemented!()
    }
}