///! A public lib for outer user call
///
use mock_derive::mock;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct DataDefineBase {
    pub biz: String,
    pub version: u32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct WorldConnectionInput {
    pub define: DataDefineBase,
    pub content: String,
    pub context: String,
}

#[mock]
pub trait WorldConnectionService {
    fn input(&self, data: WorldConnectionInput) -> Result<u64, String>;
    fn input_batch(&self, batch: Vec<WorldConnectionInput>) -> Result<u64, String>;
    fn converter_callback(&self) -> Result<u64, String>;
    fn query(&self);
}