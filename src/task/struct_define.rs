use data::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteInfo {
    pub instance: Instance,
    pub maps: Vec<Relation>,
}




#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialFinished {
    pub succeeded_id: Vec<u128>,
    pub errors: Vec<String>,
}

pub enum DataType {
    Store = 1,
    Route = 2,
    Dispatch = 3,
    Convert = 4,
    ParallelBatch = 11,
    QueueBatch = 12,
}