use super::*;

pub struct MappingDaoService;

impl MappingDao for MappingDaoService {
    /// The relations will be cached
    fn get_relations(_from: &Thing) -> Result<Vec<Mapping>> {
        unimplemented!()
    }
}