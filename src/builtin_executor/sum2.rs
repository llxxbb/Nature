// use std::collections::HashMap;
// use std::str::FromStr;
//
// use nature_common::{ConverterParameter, ConverterReturned, Executor, get_para_and_key_from_para, Instance, is_default, NatureError};
// use nature_common::Result;
//
// /// items can't be repeated
// /// detail always save due to recognize the repeated item.
// pub fn sum(input: &ConverterParameter) -> ConverterReturned {
//     // get setting
//     let cfg = match serde_json::from_str::<Setting>(&input.cfg) {
//         Ok(cfg) => cfg,
//         Err(err) => {
//             warn!("error setting: {}", &input.cfg);
//             return ConverterReturned::LogicalError(err.to_string());
//         }
//     };
//     // make input
//     let (items, ws) = match cfg {
//         Setting::FromUp(part, ws) => match one2vec(&input.from.content) {
//             Ok(rtn) => (rtn, ws),
//             Err(e) => return ConverterReturned::LogicalError(e.to_string())
//         },
//         Setting::Batch(ws) => match serde_json::from_str::<Vec<Item>>(&input.from.content) {
//             Ok(rtn) => (rtn, ws),
//             Err(e) => {
//                 let msg = format!("builtin-sum : input format error. {}", e);
//                 return ConverterReturned::LogicalError(msg);
//             }
//         },
//     };
//     // get downstream content
//     let content = match &input.last_state {
//         None => new_content(num, &key),
//         Some(o_i) => {
//             let mut content = match serde_json::from_str::<Content>(&o_i.content) {
//                 Err(err) => return ConverterReturned::LogicalError(err.to_string()),
//                 Ok(content) => content
//             };
//             match content.detail.insert(key.to_string(), num) {
//                 None => { content.total += num; }
//                 Some(o_v) => {
//                     let mode: &str = &cfg.when_same;
//                     match mode {
//                         "" => { content.detail.insert(key, o_v); }
//                         "old" => { content.detail.insert(key, o_v); }
//                         "new" => { content.total += num - o_v; }
//                         "sum" => {
//                             content.detail.insert(key, num + o_v);
//                             content.total += num;
//                         }
//                         "min" => if o_v < num { content.detail.insert(key, o_v); } else { content.total += num - o_v; }
//                         "max" => if o_v > num { content.detail.insert(key, o_v); } else { content.total += num - o_v; }
//                         _ => {
//                             let msg = format!("unknown `when_same` property: {}", mode);
//                             return ConverterReturned::LogicalError(msg);
//                         }
//                     }
//                 }
//             }
//             content
//         }
//     };
//     // make return instance
//     let mut ins = Instance::default();
//     ins.content = match serde_json::to_string(&content) {
//         Ok(s) => s,
//         Err(err) => return ConverterReturned::LogicalError(err.to_string())
//     };
//     ins.id = input.from.id;
//     ConverterReturned::Instances(vec![ins])
// }
//
// fn one2vec(para: &str, idx: &Vec<u8>, value: &str) -> Result<Vec<Item>> {
//     // prepare parameter
//     let (key, _) = match get_para_and_key_from_para(para, idx) {
//         Ok(rtn) => rtn,
//         Err(err) => {
//             let msg = format!("builtin-sum : get key from para error. {}", err.to_string());
//             return Err(NatureError::VerifyError(msg));
//         }
//     };
//     let num = match usize::from_str(value) {
//         Err(err) => {
//             let msg = format!("builtin-sum : the value be used to sum is not a number. {}", err.to_string());
//             return Err(NatureError::VerifyError(msg));
//         }
//         Ok(num) => num
//     };
//     Ok(vec![Item {
//         key,
//         value: num,
//     }])
// }
//
//
// fn new_content(num: usize, key: &str) -> Content {
//     let mut detail: HashMap<String, usize> = HashMap::new();
//     detail.insert(key.to_string(), num);
//     Content {
//         detail,
//         total: num,
//     }
// }
//
//
// #[derive(Serialize, Deserialize)]
// enum Setting {
//     /// sum upstream
//     /// `Vec<u8>` which part of para you want to sum which will become to key of the item for summing,
//     /// the value is the index of para.
//     FromUp(Vec<u8>, WhenSame),
//     /// The `Instance.content` will the json value of `Vec<Item>`
//     Batch(WhenSame),
// }
//
// #[derive(Serialize, Deserialize)]
// struct Item {
//     key: String,
//     value: usize,
// }
//
// /// hwo to process the same item's value
// ///     "" | old: remain the old value
// ///     new: use new value replace the old value
// ///     sum: use the old + new value to replace the old value
// ///     min: use min(old,new) value to replace the old value
// ///     max: use max(old,new) value to replace the old value
// #[derive(Serialize, Deserialize)]
// enum WhenSame {
//     Old,
//     New,
//     Sum,
//     Min,
//     Max,
// }
//
//
// /// the needed target data format is : [key],[value1],[value2],[value3],...
// /// for example: item1,2,100  // the custom bought 2 item1 and paid $100.
// #[derive(Deserialize, Serialize)]
// struct Content {
//     detail: HashMap<String, usize>,
//     total: usize,
// }
//
