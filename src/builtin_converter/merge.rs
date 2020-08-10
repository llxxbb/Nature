use std::collections::HashMap;
use std::str::FromStr;

use nature_common::{ConverterParameter, ConverterReturned, get_para_and_key_from_para, Instance, is_default, NatureError};
use nature_common::Result;

/// items can't be repeated
/// detail always save due to recognize the repeated item.
pub fn merge(input: &ConverterParameter) -> ConverterReturned {
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
    let items = match &cfg.key {
        KeyType::Para(part) => match one_to_vec(&input.from.para, &part, &input.from.content) {
            Ok(rtn) => rtn,
            Err(e) => return ConverterReturned::LogicalError(e.to_string())
        },
        KeyType::VecTuple => match serde_json::from_str::<Vec<Item>>(&input.from.content) {
            Ok(rtn) => rtn,
            Err(e) => {
                let msg = format!("builtin-merge : input format error. {}", e);
                warn!("{}, content: {}", msg, input.from.content);
                return ConverterReturned::LogicalError(msg);
            }
        },
        KeyType::None => match serde_json::from_str::<Vec<String>>(&input.from.content) {
            Ok(rtn) => {
                let mut items: Vec<Item> = vec![];
                for one in rtn {
                    let value = match i64::from_str(&one) {
                        Ok(num) => num,
                        Err(e) => {
                            let msg = format!("builtin-merge : input format error. {}", e);
                            warn!("{}, content: {}", msg, input.from.content);
                            return ConverterReturned::LogicalError(msg);
                        }
                    };
                    items.push(Item { key: "ignore".to_string(), value });
                }
                items
            }
            Err(e) => {
                let msg = format!("builtin-merge : input format error. {}", e);
                warn!("{}, content: {}", msg, input.from.content);
                return ConverterReturned::LogicalError(msg);
            }
        }
    };
    // init result for return
    let mut content = match &input.last_state {
        None => Content::default(),
        Some(o_i) => match serde_json::from_str::<Content>(&o_i.content) {
            Err(err) => return ConverterReturned::LogicalError(err.to_string()),
            Ok(content) => content
        }
    };
    let loop_limit = 1000;
    let mut counter = 0;
    // summary
    for one in items {
        merge_one(&cfg, &mut content, one);
        counter += 1;
        if counter == loop_limit {
            counter = 0;
            top(&cfg, &mut content);
        }
    }
    // top it
    top(&cfg, &mut content);

    // make return instance
    let mut ins = Instance::default();
    ins.content = if cfg.key == KeyType::None {
        match content.detail.get("ignore") {
            Some(s) => s.to_string(),
            None => 0.to_string(),
        }
    } else {
        if cfg.sum_all {
            match serde_json::to_string(&content) {
                Ok(s) => s,
                Err(err) => return ConverterReturned::LogicalError(err.to_string())
            }
        } else {
            match serde_json::to_string(&content.detail) {
                Ok(s) => s,
                Err(err) => return ConverterReturned::LogicalError(err.to_string())
            }
        }
    };
    ins.id = input.from.id;
    ins.para = input.from.para.clone();
    ConverterReturned::Instances(vec![ins])
}

fn top(cfg: &Setting, mut content: &mut Content) {
    match cfg.top {
        TopMode::None => (),
        TopMode::MaxTop(top) => top_it(top, true, &mut content),
        TopMode::MinTop(top) => top_it(top, false, &mut content),
    }
}

fn top_it(_top: u16, _max: bool, _content: &mut Content) {}

fn merge_one(cfg: &Setting, content: &mut Content, one: Item) {
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

fn one_to_vec(para: &str, idx: &Vec<u8>, value: &str) -> Result<Vec<Item>> {
// prepare parameter
    let (key, _) = match get_para_and_key_from_para(para, idx) {
        Ok(rtn) => rtn,
        Err(err) => {
            let msg = format!("builtin-merge : get key from para error. {}", err.to_string());
            warn!("{}, para: {}, index:{:?}", msg, para, idx);
            return Err(NatureError::VerifyError(msg));
        }
    };
    let num = match i64::from_str(value) {
        Err(err) => {
            let msg = format!("builtin-merge : the value be used to sum is not a number. {}", err.to_string());
            warn!("{}, value: {}", msg, value);
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
    // if 0 all items will input to detail,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    top: TopMode,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
enum KeyType {
    /// `Vec<u8>` which part of para you want to sum， it will become to the key of the item,
    Para(Vec<u8>),
    /// The `Instance.content` will the json value of `Vec<Item>`
    VecTuple,
    /// The `Instance.content` will the json value of `Vec<i64>`, no key needed
    None,
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::None
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
enum TopMode {
    MaxTop(u16),
    MinTop(u16),
    None,
}

impl Default for TopMode {
    fn default() -> Self {
        TopMode::None
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
                top: Default::default(),
            }).unwrap(),
        };
        dbg!(&input.cfg);
        if let ConverterReturned::LogicalError(e) = merge(&input) {
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
                top: Default::default(),
            }).unwrap(),
        };
        if let ConverterReturned::LogicalError(e) = merge(&input) {
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
                top: Default::default(),
            }).unwrap(),
        };
        match merge(&input) {
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
            top: Default::default(),
        }).unwrap();
        match merge(&input) {
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
            top: Default::default(),
        }).unwrap();
        dbg!(&input.cfg);
        match merge(&input) {
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
            top: Default::default(),
        }).unwrap();
        match merge(&input) {
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
            top: Default::default(),
        }).unwrap();
        match merge(&input) {
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
            top: Default::default(),
        }).unwrap();
        match merge(&input) {
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
            cfg: r#"{"key":"VecTuple"}"#.to_string(),
        };
        if let ConverterReturned::LogicalError(e) = merge(&input) {
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
            cfg: r#"{"key":"VecTuple"}"#.to_string(),
        };
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
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
            cfg: r#"{"key":"VecTuple"}"#.to_string(),
        };

        // mode sum
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":212}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode old
        input.cfg = r#"{"key":"VecTuple","when_same":"Old"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":112}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode New
        input.cfg = r#"{"key":"VecTuple","when_same":"New"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":100}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode Max
        input.cfg = r#"{"key":"VecTuple","when_same":"Max"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":112}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode Min
        input.cfg = r#"{"key":"VecTuple","when_same":"Min"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"{"a":100}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };
    }
}

#[cfg(test)]
mod content_none_key_test {
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
        if let ConverterReturned::LogicalError(e) = merge(&input) {
            assert_eq!(e.contains("input format error"), true);
        } else {
            panic!("should return error");
        }
    }

    #[test]
    fn one() {
        let input = ConverterParameter {
            from: {
                let mut ins = Instance::default();
                ins.content = r#"["112"]"#.to_string();
                ins
            },
            last_state: None,
            task_id: "".to_string(),
            master: None,
            cfg: "".to_string(),
        };
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, r#"112"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };
    }

    #[test]
    fn mode() {
        let mut input = ConverterParameter {
            from: {
                let mut ins = Instance::default();
                ins.content = r#"["112","100"]"#.to_string();
                ins
            },
            last_state: None,
            task_id: "".to_string(),
            master: None,
            cfg: "".to_string(),
        };

        // mode sum
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, "212");
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode old
        input.cfg = r#"{"when_same":"Old"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, "112");
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode New
        input.cfg = r#"{"when_same":"New"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, "100");
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode Max
        input.cfg = r#"{"when_same":"Max"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, "112");
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // mode Min
        input.cfg = r#"{"when_same":"Min"}"#.to_string();
        let _ = if let ConverterReturned::Instances(rtn) = merge(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.content, "100");
            rtn.clone()
        } else {
            panic!("return error result");
        };
    }
}
