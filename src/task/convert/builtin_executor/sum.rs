use std::collections::HashMap;
use std::str::FromStr;

use nature_common::{ConverterParameter, ConverterReturned, get_para_and_key_from_para, Instance, is_default};

#[derive(Serialize, Deserialize)]
struct Setting {
    /// which part of para you want to sum, the value is the index of para.
    para_part: u8,
    /// new same item will cover the old one
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    write_over: bool,
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
    let (key, para) = match get_para_and_key_from_para(&input.from.para, &vec![cfg.para_part]) {
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
                    match cfg.write_over {
                        true => content.total += num - o_v,
                        false => { content.detail.insert(key, o_v); }
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
    fn para_part() {
        let set = Setting {
            para_part: 2,
            write_over: false,
        };
        assert_eq!(serde_json::to_string(&set).unwrap(), r#"{"para_part":2}"#);
    }

    #[test]
    fn write_over() {
        let set = Setting {
            para_part: 2,
            write_over: true,
        };
        assert_eq!(serde_json::to_string(&set).unwrap(), r#"{"para_part":2,"write_over":true}"#);
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
            task_id: vec![],
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
            task_id: vec![],
            master: None,
            cfg: r#"{"para_part":1}"#.to_string(),
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
            task_id: vec![],
            master: None,
            cfg: r#"{"para_part":1}"#.to_string(),
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
            task_id: vec![],
            master: None,
            cfg: r#"{"para_part":1}"#.to_string(),
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

        // repeat should over write the old
        let mut from = Instance::default();
        from.data.content = "7".to_string();
        from.para = "a/e/c".to_string();
        let input = ConverterParameter {
            from,
            last_state: Some(last.clone()),
            task_id: vec![],
            master: None,
            cfg: r#"{"para_part":1,"write_over":true}"#.to_string(),
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
    }
}

