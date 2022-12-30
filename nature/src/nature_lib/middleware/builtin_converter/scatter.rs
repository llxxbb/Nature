use serde_json::value::RawValue;

use crate::domain::*;
use crate::util::*;

/// Setting is a json, include the following properties:
/// each you defined dimensions will be output as `Instance.para`
#[derive(Serialize, Deserialize)]
struct Setting {
    /// default is "/"
    #[serde(skip_serializing_if = "is_default_para_separator")]
    #[serde(default = "default_para_separator")]
    pub dimension_separator: String,
}

/// each item in the table which will be scattered
#[derive(Serialize, Deserialize, Clone)]
struct Item<'a> {
    /// include all dimension, separator is defined in Setting
    key: &'a str,
    /// each split dimension will copy this value.
    #[serde(borrow)]
    value: &'a RawValue,
}

#[derive(Clone)]
struct MiddleResult<'a>(&'a str, Vec<Item<'a>>);

/// Suggestion:
/// - use-upstream-id to avoid result scatter
pub fn scatter(para: &ConverterParameter) -> ConverterReturned {
    // check setting
    let cfg = if para.cfg.eq("") {
        "{}"
    } else {
        &para.cfg
    };
    let set = serde_json::from_str::<Setting>(cfg);
    if let Err(e) = set {
        let msg = format!("setting error : {:?}", e.to_string());
        return ConverterReturned::LogicalError { msg: msg };
    }
    let set = set.unwrap();

    // check input content
    let input = serde_json::from_str::<Vec<Item>>(&para.from.content);
    if let Err(e) = input {
        let msg = format!("instance content error : {:?}", e.to_string());
        return ConverterReturned::LogicalError { msg: msg };
    }

    // process split
    let mut rtn: Vec<Instance> = vec![];
    let input = input.unwrap();
    let need_replace = !set.dimension_separator.eq(&*SEPARATOR_INS_PARA);
    for one in input {
        let mut ins = Instance::default();
        if need_replace {
            ins.path.para = one.key.replace(&set.dimension_separator, &*SEPARATOR_INS_PARA);
        } else {
            ins.path.para = one.key.to_string();
        }
        ins.content = one.value.to_string();
        rtn.push(ins);
    }
    // return result
    ConverterReturned::Instances { ins: rtn }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scatter_test() {
        let mut content: Vec<KV> = vec![];
        content.push(KV::new("class5|name1|subject1", 92));
        content.push(KV::new("class5|name1|subject2", 85));
        content.push(KV::new("class5|name1|subject3", 99));

        let mut input_instance = Instance::default();
        input_instance.content = serde_json::to_string(&content).unwrap();

        let setting = Setting {
            dimension_separator: "|".to_string(),
        };

        let para = ConverterParameter {
            from: input_instance,
            last_state: None,
            task_id: 0,
            master: None,
            cfg: serde_json::to_string(&setting).unwrap(),
        };

        let rtn = scatter(&para);
        if let ConverterReturned::Instances { ins } = rtn {
            assert_eq!(ins.len(), 3);
            let one = &ins[0];
            assert_eq!(one.path.para, "class5/name1/subject1");
            assert_eq!(one.content, "92");
        };
    }

    #[test]
    fn setting_default() {
        let set = Setting {
            dimension_separator: "/".to_string(),
        };
        let json = serde_json::to_string(&set).unwrap();
        let cmp = r#"{}"#;
        assert_eq!(json, cmp);
        let set = serde_json::from_str::<Setting>(&json).unwrap();
        assert_eq!(set.dimension_separator, "/")
    }

    #[derive(Serialize)]
    struct KV {
        pub key: String,
        pub value: i32,
    }

    impl KV {
        pub fn new(key: &str, value: i32) -> Self {
            KV {
                key: key.to_string(),
                value,
            }
        }
    }
}