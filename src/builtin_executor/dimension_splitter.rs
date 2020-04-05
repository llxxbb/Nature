use std::ops::Range;

use nature_common::{ConverterParameter, ConverterReturned};

/// Data input format required: Vec<(Key,Value)>
/// - Key: is String type, include all dimension, separator is defined in Setting
/// - Value: is Any type, each split dimension will copy this value.
///
/// Setting is a json, include the following properties:
/// - dimension-separator: default is "/"
/// - wanted-dimension: array of array to store dimension index. for example: [1,2][1,3].
/// each you defined dimensions will be output as `Instance.para`
///
/// Suggestion:
/// - use-upstream-id to avoid result scatter
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

pub fn dimension_split(para: &ConverterParameter) -> ConverterReturned {
    // para.
    unimplemented!()
}