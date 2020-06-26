use std::collections::HashMap;
use std::str::FromStr;

use nature_common::{ConverterParameter, ConverterReturned, get_para_and_key_from_para, Instance, is_default};

#[derive(Serialize, Deserialize)]
struct Setting {
    /// which part of para you want to sum, the value is the index of para.
    key_from_para: Vec<u8>,

    /// hwo to process the same item's value
    ///     "" | old: remain the old value
    ///     new: use new value replace the old value
    ///     sum: use the old + new value to replace the old value
    ///     min: use min(old,new) value to replace the old value
    ///     max: use max(old,new) value to replace the old value
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    when_same: String,
}

#[derive(Deserialize, Serialize)]
struct Content {
    detail: HashMap<String, usize>,
    total: usize,
}

/// items can't be repeated
/// detail always save due to recognize the repeated item.
pub fn sum(input: &ConverterParameter) -> ConverterReturned {
    // get setting
    let cfg = match serde_json::from_str::<Setting>(&input.cfg) {
        Ok(cfg) => cfg,
        Err(err) => {
            warn!("error setting: {}", &input.cfg);
            return ConverterReturned::LogicalError(err.to_string());
        }
    };
    // get upstream num
    let num = match usize::from_str(&input.from.content) {
        Err(err) => return ConverterReturned::LogicalError(err.to_string()),
        Ok(num) => num
    };
    // prepare parameter
    let (key, para) = match get_para_and_key_from_para(&input.from.para, &cfg.key_from_para) {
        Ok(rtn) => rtn,
        Err(err) => return ConverterReturned::LogicalError(err.to_string())
    };
    // get downstream content
    let content = match &input.last_state {
        None => new_content(num, &key),
        Some(o_i) => {
            let mut content = match serde_json::from_str::<Content>(&o_i.content) {
                Err(err) => return ConverterReturned::LogicalError(err.to_string()),
                Ok(content) => content
            };
            match content.detail.insert(key.to_string(), num) {
                None => { content.total += num; }
                Some(o_v) => {
                    let mode: &str = &cfg.when_same;
                    match mode {
                        "" => { content.detail.insert(key, o_v); }
                        "old" => { content.detail.insert(key, o_v); }
                        "new" => { content.total += num - o_v; }
                        "sum" => {
                            content.detail.insert(key, num + o_v);
                            content.total += num;
                        }
                        "min" => if o_v < num { content.detail.insert(key, o_v); } else { content.total += num - o_v; }
                        "max" => if o_v > num { content.detail.insert(key, o_v); } else { content.total += num - o_v; }
                        _ => {
                            let msg = format!("unknown `when_same` property: {}", mode);
                            return ConverterReturned::LogicalError(msg);
                        }
                    }
                }
            }
            content
        }
    };
    // make return instance
    let mut ins = Instance::default();
    ins.content = match serde_json::to_string(&content) {
        Ok(s) => s,
        Err(err) => return ConverterReturned::LogicalError(err.to_string())
    };
    ins.para = para;
    ins.id = input.from.id;
    ConverterReturned::Instances(vec![ins])
}

fn new_content(num: usize, key: &str) -> Content {
    let mut detail: HashMap<String, usize> = HashMap::new();
    detail.insert(key.to_string(), num);
    Content {
        detail,
        total: num,
    }
}

#[cfg(test)]
mod sum_setting_test {
    use super::*;

    #[test]
    fn key_from_para() {
        let set = Setting {
            key_from_para: vec![2],
            when_same: "".to_string(),
        };
        assert_eq!(serde_json::to_string(&set).unwrap(), r#"{"key_from_para":[2]}"#);
    }

    #[test]
    fn write_over() {
        let set = Setting {
            key_from_para: vec![2],
            when_same: "new".to_string(),
        };
        assert_eq!(serde_json::to_string(&set).unwrap(), r#"{"key_from_para":[2],"when_same":"new"}"#);
    }
}

#[cfg(test)]
mod sum_test {
    use super::*;

    #[test]
    fn input_nothing() {
        let input = ConverterParameter {
            from: Default::default(),
            last_state: None,
            task_id: "".to_string(),
            master: None,
            cfg: "".to_string(),
        };
        if let ConverterReturned::LogicalError(e) = sum(&input) {
            assert_eq!(e, "EOF while parsing a value at line 1 column 0");
        } else {
            panic!("return error result");
        }
    }

    #[test]
    fn normal() {
        // one input
        let mut from = Instance::default();
        from.data.content = "5".to_string();
        from.para = "a/b/c".to_string();
        let input = ConverterParameter {
            from,
            last_state: None,
            task_id: "".to_string(),
            master: None,
            cfg: r#"{"key_from_para":[1]}"#.to_string(),
        };
        let last = if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.para, "a/c");
            assert_eq!(rtn.content, r#"{"detail":{"b":5},"total":5}"#);
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // another input
        let mut from = Instance::default();
        from.data.content = "6".to_string();
        from.para = "a/e/c".to_string();
        let input = ConverterParameter {
            from,
            last_state: Some(last.clone()),
            task_id: "".to_string(),
            master: None,
            cfg: r#"{"key_from_para":[1]}"#.to_string(),
        };
        let last = if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.para, "a/c");
            dbg!(&rtn.content);
            assert!(rtn.content.contains(r#""total":11"#));
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // repeat should not cover the old
        let mut from = Instance::default();
        from.data.content = "7".to_string();
        from.para = "a/e/c".to_string();
        let input = ConverterParameter {
            from,
            last_state: Some(last.clone()),
            task_id: "".to_string(),
            master: None,
            cfg: r#"{"key_from_para":[1]}"#.to_string(),
        };
        let last = if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.para, "a/c");
            dbg!(&rtn.content);
            assert!(rtn.content.contains(r#""e":6"#));
            assert!(rtn.content.contains(r#""total":11"#));
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // repeat should not cover the min
        let mut from = Instance::default();
        from.data.content = "7".to_string();
        from.para = "a/e/c".to_string();
        let input = ConverterParameter {
            from,
            last_state: Some(last.clone()),
            task_id: "".to_string(),
            master: None,
            cfg: r#"{"key_from_para":[1],"when_same":"min"}"#.to_string(),
        };
        let last = if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.para, "a/c");
            dbg!(&rtn.content);
            assert!(rtn.content.contains(r#""e":6"#));
            assert!(rtn.content.contains(r#""total":11"#));
            rtn.clone()
        } else {
            panic!("return error result");
        };

        // repeat should over write the new
        let mut from = Instance::default();
        from.data.content = "7".to_string();
        from.para = "a/e/c".to_string();
        let input = ConverterParameter {
            from,
            last_state: Some(last.clone()),
            task_id: "".to_string(),
            master: None,
            cfg: r#"{"key_from_para":[1],"when_same":"new"}"#.to_string(),
        };
        if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.para, "a/c");
            dbg!(&rtn.content);
            assert!(rtn.content.contains(r#""e":7"#));
            assert!(rtn.content.contains(r#""total":12"#));
        } else {
            panic!("return error result");
        };

        // repeat should over write the max
        let mut from = Instance::default();
        from.data.content = "7".to_string();
        from.para = "a/e/c".to_string();
        let input = ConverterParameter {
            from,
            last_state: Some(last.clone()),
            task_id: "".to_string(),
            master: None,
            cfg: r#"{"key_from_para":[1],"when_same":"max"}"#.to_string(),
        };
        if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.para, "a/c");
            dbg!(&rtn.content);
            assert!(rtn.content.contains(r#""e":7"#));
            assert!(rtn.content.contains(r#""total":12"#));
        } else {
            panic!("return error result");
        };

        // repeat should sum with the sum
        let mut from = Instance::default();
        from.data.content = "7".to_string();
        from.para = "a/e/c".to_string();
        let input = ConverterParameter {
            from,
            last_state: Some(last.clone()),
            task_id: "".to_string(),
            master: None,
            cfg: r#"{"key_from_para":[1],"when_same":"sum"}"#.to_string(),
        };
        if let ConverterReturned::Instances(rtn) = sum(&input) {
            let rtn = &rtn[0];
            assert_eq!(rtn.para, "a/c");
            dbg!(&rtn.content);
            assert!(rtn.content.contains(r#""e":13"#));
            assert!(rtn.content.contains(r#""total":18"#));
        } else {
            panic!("return error result");
        };
    }
}

