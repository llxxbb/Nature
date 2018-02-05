///! A public lib for outer user call
///
pub struct DataDefineBase {
    pub biz: String,
    pub version: u32,
}

pub struct Data {
    pub define: DataDefineBase,
    pub content: String,
    pub context: String,
}


pub enum WorldConnectionResult {
    OkR(u32),
    Err(String),
}

pub trait WorldConnectionService {
    fn input(data: Data) -> WorldConnectionResult;
    fn input_batch(batch: Vec<Data>) -> WorldConnectionResult;
    fn converter_callback() -> WorldConnectionResult;
    fn query();
}