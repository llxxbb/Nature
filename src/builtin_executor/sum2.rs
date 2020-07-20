use std::collections::HashMap;

use nature_common::{Executor, is_default};

#[derive(Serialize, Deserialize)]
enum Setting {
    /// sum upstream
    /// `Vec<u8>` which part of para you want to sum, the value is the index of para.
    FromUp(Vec<u8>, WhenSame),
    /// the data that used to sum need load from Instance table
    Table(BatchMode, WhenSame),
}

/// hwo to process the same item's value
///     "" | old: remain the old value
///     new: use new value replace the old value
///     sum: use the old + new value to replace the old value
///     min: use min(old,new) value to replace the old value
///     max: use max(old,new) value to replace the old value
#[derive(Serialize, Deserialize)]
enum WhenSame {
    Old,
    New,
    Sum,
    Min,
    Max,
}

/// when used this mode the target `MetaType` must be `Multi`
#[derive(Serialize, Deserialize)]
struct BatchMode {
    /// the prefix of `ins_key`
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    key_like: String,
    #[serde(skip_serializing_if = "is_100")]
    #[serde(default = "default_100")]
    page_size: u16,
    /// where to get the time range from the `Instance'para` which used to load data from Instance table
    time_part: Vec<u8>,
    /// before process, correct the format of the data inputted.
    /// the needed target data format is : [key],[value1],[value2],[value3],...
    /// each value will be used to sum
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pre_filters: Vec<Executor>,
    /// each out is corresponding to a value tu sum that must be defined in `meta` table
    out: Vec<String>,
}

fn is_100(size: &u16) -> bool {
    if *size == 100 {
        true
    } else {
        false
    }
}

fn default_100() -> u16 { 100 }

/// the needed target data format is : [key],[value1],[value2],[value3],...
/// for example: item1,2,100  // the custom bought 2 item1 and paid $100.


#[derive(Deserialize, Serialize)]
struct Content {
    detail: HashMap<String, usize>,
    total: usize,
}

#[cfg(test)]
mod test {
    use nature_common::Protocol;

    use super::*;

    #[test]
    #[ignore]
    fn json() {
        let result = serde_json::to_string(&Setting::FromUp(vec![0], WhenSame::Old)).unwrap();
        assert_eq!(result, r#"{"FromUp":[[0],"Old"]}"#);

        let result = serde_json::to_string(&Setting::Table(BatchMode {
            key_like: "abc".to_string(),
            page_size: default_100(),
            time_part: vec![0, 1],
            pre_filters: vec![Executor {
                protocol: Protocol::LocalRust,
                url: "a:b".to_string(),
                settings: "".to_string(),
            }],
            out: vec!["hello".to_string()],
        }, WhenSame::Sum)).unwrap();
        assert_eq!(result, r#"{"FromUp":[[0],"Old"]}"#)
    }
}
