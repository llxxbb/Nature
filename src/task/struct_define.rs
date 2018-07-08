#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialFinished {
    pub succeeded_id: Vec<u128>,
    pub errors: Vec<String>,
}

