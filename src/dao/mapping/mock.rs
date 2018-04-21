use super::*;

pub struct MappingDaoService;

impl MappingDao for MappingDaoService {
    fn get_relations(_instance: &Instance) -> Result<Option<RouteInfo>> {
        unimplemented!()
    }
}