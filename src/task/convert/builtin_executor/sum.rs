use std::collections::HashMap;
use std::str::FromStr;

use nature_common::{ConverterParameter, ConverterReturned, get_para_and_key_from_para, Instance, is_default};

#[derive(Serialize, Deserialize)]
struct Setting {
    /// array of the `para` index. for example: [2,1].
    wanted_para: Vec<u8>,
    /// new same item will cover the old one
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    item_cover: bool,
}

#[derive(Deserialize, Serialize)]
struct Content {
    detail: HashMap<String, usize>,
    total: usize,
}

/// items can't be repeated
/// detail always save due to recognize the repeated item.
pub fn sum(cp: &ConverterParameter) -> ConverterReturned {
    // get setting
    let cfg = match serde_json::from_str::<Setting>(&cp.cfg) {
        Ok(cfg) => cfg,
        Err(err) => {
            warn!("error setting: {}", &cp.cfg);
            return ConverterReturned::LogicalError(err.to_string());
        }
    };
    // get upstream num
    let num = match usize::from_str(&cp.from.content) {
        Err(err) => return ConverterReturned::LogicalError(err.to_string()),
        Ok(num) => num
    };
    // prepare parameter
    let (key, para) = match get_para_and_key_from_para(&cp.from.para, &cfg.wanted_para) {
        Ok(rtn) => rtn,
        Err(err) => return ConverterReturned::LogicalError(err.to_string())
    };
    // get downstream content
    let content = match &cp.last_state {
        None => new_content(num, &key),
        Some(o_i) => {
            let mut content = match serde_json::from_str::<Content>(&o_i.content) {
                Err(err) => return ConverterReturned::LogicalError(err.to_string()),
                Ok(content) => content
            };
            match content.detail.insert(key.to_string(), num) {
                None => { content.total += num; }
                Some(o_v) => match cfg.item_cover {
                    true => (),
                    false => { content.detail.insert(key, o_v); }
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
    ins.id = cp.from.id;
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
mod test {
    use super::*;

    #[test]
    fn test_setting() {
        let set = Setting {
            wanted_para: vec![3, 1],
            item_cover: false,
        };
        assert_eq!(serde_json::to_string(&set).unwrap(), r#"{"wanted_para":[3,1]}"#);
    }
}

