use super::*;

pub struct OneStepFlowDaoImpl;

impl OneStepFlowDaoTrait for OneStepFlowDaoImpl {
    fn get_relations(_from: &Thing) -> Result<Vec<Relation>> {
        unimplemented!()
    }
}