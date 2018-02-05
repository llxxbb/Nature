use biz::WorldConnectionService;
use biz::Data;
use biz::WorldConnectionResult;

///! World Connection Service provider
pub struct Server {}


impl WorldConnectionService for Server {
    fn input(_data: Data) -> WorldConnectionResult {
        unimplemented!()
    }

    fn input_batch(_batch: Vec<Data>) -> WorldConnectionResult {
        unimplemented!()
    }

    fn converter_callback() -> WorldConnectionResult {
        unimplemented!()
    }

    fn query() {
        unimplemented!()
    }
}
