use nature_common::{ConverterParameter, ConverterReturned, default_para_separator, is_default, is_default_para_separator};

#[derive(Serialize, Deserialize)]
struct Setting {
    /// default is "/"
    #[serde(skip_serializing_if = "is_default_para_separator")]
    #[serde(default = "default_para_separator")]
    para_separator: String,
    /// array of the `para` index. for example: [2,1].
    wanted_para: Vec<u8>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    need_detail: bool,
}

// struct Content {
//     detail: Vec<(String, usize)>,
//     total: usize,
// }


/// Requirement:
/// - value that will be summed must be `usize` type.
/// - `Instance`'s content should be only contain the the value
/// - downstream `Instance`:
///   - id will upstream's id
///
///
pub fn sum_allow_repeat(_para: &ConverterParameter) -> ConverterReturned {
    unimplemented!()
}
