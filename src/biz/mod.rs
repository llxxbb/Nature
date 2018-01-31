///! A public lib for outer interface to use World Connection

pub struct DataDefineBase {
    pub biz: String,
    pub version: u32,
}

pub struct WorldConnectionData {
    pub define: DataDefineBase,
    pub content: String,
    pub context: String,
}

pub struct WorldConnectionResult {
    pub status: String,
    pub err_msg: String,
    pub serial_number: u64,
}

pub trait Teller {
    fn input(d: WorldConnectionData) -> WorldConnectionResult;
}