use std::ops::Range;

use nature_common::{ConverterParameter, ConverterReturned};

/// The `Instance` format required
/// - must be parametric, so that can be query out your need from the pile
/// - the content must be only contain a number, so that the `SimpleCounter` will recognize it and counter it
/// put the condition to to context include
/// - B.inPara = a,b,c...     // the sequence is important
/// - B.meta = some/meta    // if it be ignored then upstream `Meta` will be used
/// - B.outPara = a,b,c...
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

pub fn dimension_split(_para: &ConverterParameter) -> ConverterReturned {
    unimplemented!()
}