use std::collections::HashMap;
use std::str::FromStr;

use nature_common::{ConverterParameter, ConverterReturned, default_para_separator, Instance, is_default, is_default_para_separator};

#[derive(Serialize, Deserialize)]
struct Setting {
    /// default is "/"
    #[serde(skip_serializing_if = "is_default_para_separator")]
    #[serde(default = "default_para_separator")]
    para_separator: String,
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
        Err(err) => return ConverterReturned::LogicalError(err.to_string())
    };
    // get upstream num
    let num = match usize::from_str(&cp.from.content) {
        Err(err) => return ConverterReturned::LogicalError(err.to_string()),
        Ok(num) => num
    };
    // TODO get key-para
    let (key, para) = ("a".to_string(), "b".to_string());
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
