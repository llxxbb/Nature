use nature_common::{ConverterParameter, ConverterReturned, default_para_separator, is_default_para_separator};

/// Requirement:
/// - value that will be summed must be `usize` type.
/// - `Instance`'s content should be only contain the the value
/// - downstream `Instance`:
///   - id will upstream's id
///
#[derive(Serialize, Deserialize)]
pub struct Setting {
    /// - dimension_separator: default is "/"
    #[serde(skip_serializing_if = "is_default_para_separator")]
    #[serde(default = "default_para_separator")]
    pub dimension_separator: String,
    /// - wanted_dimension: array of array to store dimension index. for example: [["meta-a",[1,2]],["meta-b",[1,3]]].
    pub wanted_dimension: Vec<(String, Vec<u8>)>,
}

pub fn sum(_para: &ConverterParameter) -> ConverterReturned {
    unimplemented!()
}
