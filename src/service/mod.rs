///! World Connection Service provider
use biz::WorldConnectionInput;
use biz::WorldConnectionService;

pub struct Service;


impl WorldConnectionService for Service {
    fn input(&self, data: WorldConnectionInput) -> Result<u64, String> {
        if data.define.biz.is_empty() {
            return Err("[biz] must not be empty!".to_string());
        }
        Ok(1)
    }
}

