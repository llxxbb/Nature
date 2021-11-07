use std::collections::HashMap;

use nature::domain::{Instance, NatureError, Result};
use nature::util::*;

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn name_to_id(para: &Vec<Instance>) -> Result<Vec<Instance>> {
    let mut map = HashMap::new();
    map.insert("class5/name1", "001");
    map.insert("class5/name2", "002");
    map.insert("class5/name3", "003");
    map.insert("class5/name4", "004");
    map.insert("class5/name5", "005");
    let mut rtn: Vec<Instance> = vec![];
    for input in para {
        let mut one = input.clone();
        let part: Vec<&str> = one.path.para.split(&*SEPARATOR_INS_PARA).collect();
        let name = part[0].to_owned() + &*SEPARATOR_INS_PARA + part[1];
        let option = map.get(&name.as_ref());
        match option {
            None => return Err(NatureError::VerifyError(format!("can't find student id for {}", name))),
            Some(id) => one.path.para = id.to_owned().to_owned() + &*SEPARATOR_INS_PARA + part[2]
        }
        rtn.push(one);
    }
    Ok(rtn)
}

