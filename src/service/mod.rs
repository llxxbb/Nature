///! World Connection Service provider
use biz::WorldConnectionInput;
use biz::WorldConnectionService;

pub static SERVICE: &WorldConnectionService = &Service {};

pub struct Service {}

impl WorldConnectionService for Service {
    fn input(&self, _data: WorldConnectionInput) -> Result<u64, String> {
        unimplemented!()
    }

    fn input_batch(&self, _batch: Vec<WorldConnectionInput>) -> Result<u64, String> {
        unimplemented!()
    }

    fn converter_callback(&self) -> Result<u64, String> {
        unimplemented!()
    }

    fn query(&self) {
        unimplemented!()
    }
}
