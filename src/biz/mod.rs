///! A public lib for outer user call
///
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

pub trait WorldConnectionService {
    fn input(&self, data: WorldConnectionInput) -> Result<u64, &str>;
    fn input_batch(&self, batch: Vec<WorldConnectionInput>) -> Result<u64, &str>;
    fn converter_callback(&self) -> Result<u64, &str>;
    fn query(&self);
}

