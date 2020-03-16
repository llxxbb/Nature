use std::ops::Range;

use nature_common::{ConverterParameter, ConverterReturned};

use crate::task::ExecutorTrait;

pub struct Counter;

#[derive(Serialize, Deserialize)]
enum Method {
    Map(MapItem),
    Range(Vec<Range<i32>>),
}

#[derive(Serialize, Deserialize)]
struct MapItem {
    pub context_filter: Vec<String>,
    pub meta: String,
}

impl ExecutorTrait for Counter {
    fn execute(&self, _executor: &str, _para: &ConverterParameter) -> ConverterReturned {
//        let mut rtn: Vec<Instance> = Vec::new();
//        let mut meta_cnt: HashMap<String, u32> = HashMap::new();
//        loop {
//
//
//            // get instance from meta
//
//            // increase counter
//
//            //
//        }
//        ConverterReturned::Instances(rtn);
        unimplemented!()
    }
}

