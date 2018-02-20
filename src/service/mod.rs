///! World Connection Service provider
use biz::WorldConnectionInput;
use biz::WorldConnectionService;

pub struct Service {}

impl WorldConnectionService for Service {
    fn input(&self, data: WorldConnectionInput) -> Result<u64, String> {
        if data.define.biz.is_empty() {
            return Err(String::from("[biz] must not be empty!"));
        }
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
