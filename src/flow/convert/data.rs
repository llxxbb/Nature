use super::*;

pub struct Converted {
    pub done_task: Carrier<ConverterInfo>,
    pub converted: Vec<Instance>,
}