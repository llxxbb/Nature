use std::ops::Range;

use nature_common::{ConverterParameter, ConverterReturned};

use crate::task::ExecutorTrait;

/// Data must be from Nature.
/// The `Instance` format required
/// - must be parametric, so that can be query out your need from the pile
/// - the content must be only contain a number, so that the `SimpleCounter` will recognize it and counter it
/// put the condition to to context include
/// - B.inPara = a,b,c...     // the sequence is important
/// - B.meta = some/meta    // if it be ignored then upstream `Meta` will be used
/// - B.outPara = a,b,c...
pub struct SimpleCounter;

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

impl ExecutorTrait for SimpleCounter {
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

