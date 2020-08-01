use std::collections::HashMap;
use std::str::FromStr;

use nature_common::{ConverterParameter, ConverterReturned, get_para_and_key_from_para, Instance, is_default, NatureError};
use nature_common::Result;

/// items can't be repeated
/// detail always save due to recognize the repeated item.
pub fn sum(input: &ConverterParameter) -> ConverterReturned {
    // get setting
    let cfg = if input.cfg.is_empty() {
        Setting::default()
    } else {
        match serde_json::from_str::<Setting>(&input.cfg) {
            Ok(cfg) => cfg,
            Err(err) => {
                warn!("error setting: {}", &input.cfg);
                return ConverterReturned::LogicalError(err.to_string());
            }
        }
    };
    // make input
    let items = match cfg.key {
        KeyType::Para(part) => match one_to_vec(&input.from.para, &part, &input.from.content) {
            Ok(rtn) => rtn,
            Err(e) => return ConverterReturned::LogicalError(e.to_string())
        },
        KeyType::VecTuple => match serde_json::from_str::<Vec<Item>>(&input.from.content) {
            Ok(rtn) => rtn,
            Err(e) => {
                let msg = format!("builtin-sum : input format error. {}", e);
                return ConverterReturned::LogicalError(msg);
            }
        },
    };
    // init result for return
    let mut content = match &input.last_state {
        None => Content::default(),
        Some(o_i) => match serde_json::from_str::<Content>(&o_i.content) {
            Err(err) => return ConverterReturned::LogicalError(err.to_string()),
            Ok(content) => content
        }
    };
    // summary
    for one in items {
        let total_change = match content.detail.insert(one.key.to_string(), one.value) {
            None => one.value,
            Some(old) => match cfg.when_same {
                WhenSame::Old => {
                    content.detail.insert(one.key.to_string(), old);
                    0
                }
                WhenSame::New => one.value - old,
                WhenSame::Sum => {
                    content.detail.insert(one.key.to_string(), one.value + old);
                    one.value
                }
                WhenSame::Min => if old < one.value {
                    content.detail.insert(one.key.to_string(), old);
                    0
                } else { one.value - old }
                WhenSame::Max => if old > one.value {
                    content.detail.insert(one.key.to_string(), old);
                    0
                } else { one.value - old }
            }
        };
        if cfg.sum_all {
            content.total += total_change;
        }
    }

    // make return instance
    let mut ins = Instance::default();
    ins.content = if cfg.sum_all {
        match serde_json::to_string(&content) {
            Ok(s) => s,
            Err(err) => return ConverterReturned::LogicalError(err.to_string())
        }
    } else {
        match serde_json::to_string(&content.detail) {
            Ok(s) => s,
            Err(err) => return ConverterReturned::LogicalError(err.to_string())
        }
    };
    ins.id = input.from.id;
    ins.para = input.from.para.clone();
    ConverterReturned::Instances(vec![ins])
}

fn one_to_vec(para: &str, idx: &Vec<u8>, value: &str) -> Result<Vec<Item>> {
// prepare parameter
    let (key, _) = match get_para_and_key_from_para(para, idx) {
        Ok(rtn) => rtn,
        Err(err) => {
            let msg = format!("builtin-sum : get key from para error. {}", err.to_string());
            return Err(NatureError::VerifyError(msg));
        }
    };
    let num = match i64::from_str(value) {
        Err(err) => {
            let msg = format!("builtin-sum : the value be used to sum is not a number. {}", err.to_string());
            return Err(NatureError::VerifyError(msg));
        }
        Ok(num) => num
    };
    Ok(vec![Item {
        key,
        value: num,
    }])
}


#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq)]
struct Setting {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    key: KeyType,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    when_same: WhenSame,
    /// Whether to give a field `total` for summing all item's value.
    /// This is useful for summing based on state-instance
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    sum_all: bool,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
enum KeyType {
    /// `Vec<u8>` which part of para you want to sum， it will become to the key of the item,
    Para(Vec<u8>),
    /// The `Instance.content` will the json value of `Vec<Item>`
    VecTuple,
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::VecTuple
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
struct Item {
    key: String,
    value: i64,
}

/// hwo to process the same item's value
///     "" | old: remain the old value
///     new: use new value replace the old value
///     sum: use the old + new value to replace the old value
///     min: use min(old,new) value to replace the old value
///     max: use max(old,new) value to replace the old value
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
enum WhenSame {
    Old,
    New,
    Sum,
    Min,
    Max,
}

impl Default for WhenSame {
    fn default() -> Self {
        WhenSame::Sum
    }
}

/// the needed target data format is : [key],[value1],[value2],[value3],...
/// for example: item1,2,100  // the custom bought 2 item1 and paid $100.
#[derive(Deserialize, Serialize, Default, Debug)]
struct Content {
    detail: HashMap<String, i64>,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    total: i64,
}


#[cfg(test)]
mod sum_setting_test {
    use super::*;

    #[test]
    fn default() {
        let set = Setting::default();
        assert_eq!(serde_json::to_string(&set).unwrap(), r#"{}"#);
    }
}

#[cfg(test)]
mod para_type_test {
    use super::*;

    #[test]
    fn no_para_input() {
        let input = ConverterParameter {
            from: Default::default(),
            last_state: None,
            task_id: "".to_string(),
            master: None,
            cfg: serde_json::to_string(&Setting {
                key: KeyType::Para(vec![1]),
                when_same: Default::default(),
                sum_all: false,
            }).unwrap(),
        };
        if let ConverterReturned::LogicalError(e) = sum(&input) {
            assert!(e.contains("get key from para error"));
        } else {
            panic!("should return error");
        }
    }

    #[test]
    fn content_empty() {
        let input = ConverterParameter {
            from: {
                let mut rtn = Instance::default();
                rtn.para = "a/b/c".to_string();
                rtn
            },
            last_state: None,
            task_id: "".to_string(),
            master: None,
            cfg: serde_json::to_string(&Setting {
                key: KeyType::Para(vec![1]),
                when_same: Default::default(),
                sum_all: false,
            }).unwrap(),
        };
        if let ConverterReturned::LogicalError(e) = sum(&input) {
            assert_eq!(true, e.contains("the value be used to sum is not a number"));
        } else {
            panic!("should return error");
        }
    }

    #[test]
    fn content_new() {
        let input = ConverterParameter {
            from: {
                let mut rtn = Instance::default();
                rtn.para = "a/b/c".to_string();
                rtn.content = "123".to_string();
                rtn
            },
            last_state: None,
            task_id: "".to_string(),
            master: None,
            cfg: serde_json::to_string(&Setting {
                key: KeyType::Para(vec![1]),
                when_same: Default::default(),
                sum_all: true,
            }).unwrap(),
        };
        match sum(&input) {
            ConverterReturned::Instances(rtn) => {
                assert_eq!(rtn[0].content, r#"{"detail":{"b":123},"total":123}"#);
            }
            _ => panic!("error")
        }
    }

    #[test]
    fn mode() {
        let mut input = ConverterParameter {
            from: {
                let mut rtn = Instance::default();
                rtn.para = "a/b/c".to_string();
                rtn.content = "100".to_string();
                rtn
            },
            last_state: Some({
                let mut rtn = Instance::default();
                rtn.content = r#"{"detail":{"b":123},"total":123}"#.to_string();
                rtn
            }),
            task_id: "".to_string(),
            master: None,
            cfg: "".to_string(),
        };
        // sum
        input.cfg = serde_json::to_string(&Setting {
            key: KeyType::Para(vec![1]),
            when_same: Default::default(),  // sum
            sum_all: true,
        }).unwrap();
        match sum(&input) {
            ConverterReturned::Instances(rtn) => {
                assert_eq!(rtn[0].content, r#"{"detail":{"b":223},"total":223}"#);
            }
            _ => panic!("error")
        }
        // old
        input.cfg = serde_json::to_string(&Setting {
            key: KeyType::Para(vec![1]),
            when_same: WhenSame::Old,
            sum_all: true,
        }).unwrap();
        match sum(&input) {
            ConverterReturned::Instances(rtn) => {
                assert_eq!(rtn[0].content, r#"{"detail":{"b":123},"total":123}"#);
            }
            _ => panic!("error")
        }
        // new
        input.cfg = serde_json::to_string(&Setting {
            key: KeyType::Para(vec![1]),
            when_same: WhenSame::New,
            sum_all: true,
        }).unwrap();
        match sum(&input) {
            ConverterReturned::Instances(rtn) => {
                assert_eq!(rtn[0].content, r#"{"detail":{"b":100},"total":100}"#);
            }
            _ => panic!("error")
        }
        // Max
        input.cfg = serde_json::to_string(&Setting {
            key: KeyType::Para(vec![1]),
            when_same: WhenSame::Max,
            sum_all: true,
        }).unwrap();
        match sum(&input) {
            ConverterReturned::Instances(rtn) => {
                assert_eq!(rtn[0].content, r#"{"detail":{"b":123},"total":123}"#);
            }
            _ => panic!("error")
        }
        // Min
        input.cfg = serde_json::to_string(&Setting {
            key: KeyType::Para(vec![1]),
            when_same: WhenSame::Min,
            sum_all: true,
        }).unwrap();
        match sum(&input) {
            ConverterReturned::Instances(rtn) => {
                assert_eq!(rtn[0].content, r#"{"detail":{"b":100},"total":100}"#);
            }
            _ => panic!("error")
        }
    }
}

#[cfg(test)]
mod content_tuple_test {
    use super::*;

    #[test]
    fn content_err() {
        let input = ConverterParameter {
            from: Default::default(),
            last_state: None,
            task_id: "".to_string(),
            master: None,
            cfg: "".to_string(),
        };
        if let ConverterReturned::LogicalError(e) = sum(&input) {
            assert_eq!(e.contains("input format error"), true);
        } else {
            panic!("should return error");
        }
    }

    #[test]
    fn one() {
        let data: Vec<(String, u64)> = vec![("a".to_string(), 112)];
        let data = serde_json::to_string(&data).unwrap();
        let input = ConverterParameter {
            from: {
                let mut ins = Instance::default();
                ins.content = data;
                ins
            },
            last_state: None,
            task_id: "".to_string(),
            master: None,
            cfg: "".to_string(),
        };
        let _ = if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":112}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };
    }

    #[test]
    fn mode() {
        let data: Vec<(String, u64)> = vec![
            ("a".to_string(), 112),
            ("a".to_string(), 100),
        ];
        let data = serde_json::to_string(&data).unwrap();
        let mut input = ConverterParameter {
            from: {
                let mut ins = Instance::default();
                ins.content = data;
                ins
            },
            last_state: None,
            task_id: "".to_string(),
            master: None,
            cfg: "".to_string(),
        };

        // mode sum
        let _ = if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":212}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode old
        input.cfg = r#"{"when_same":"Old"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":112}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode New
        input.cfg = r#"{"when_same":"New"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":100}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode Max
        input.cfg = r#"{"when_same":"Max"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":112}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode Min
        input.cfg = r#"{"when_same":"Min"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":100}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };
    }
}

