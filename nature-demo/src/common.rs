use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use reqwest::blocking::Client;
use serde::Serialize;

use nature::domain::{InsCond, Instance, NatureError, NoIdCond, Result};

lazy_static! {
    pub static ref CLIENT : Client = Client::new();
}

pub static URL_INPUT: &str = "http://localhost:8080/input";
pub static URL_GET_BY_ID: &str = "http://localhost:8080/get/byId";
pub static URL_GET_BY_META: &str = "http://localhost:8080/get_by_key_range";

pub fn send_instance(ins: &Instance) -> Result<u64> {
    let response = CLIENT.post(URL_INPUT).json(&ins).send();
    let id_s: String = response.unwrap().text().unwrap();
    if id_s.contains("Err") {
        return Err(NatureError::VerifyError(id_s));
    }
    serde_json::from_str(&id_s)?
}

pub fn get_by_id(cond: &InsCond) -> Option<Instance> {
    // let rtn = CLIENT.post(&*GET_BY_META).json(cond).send().await?.json::<ConverterReturned>().await?;
    let response = CLIENT.post(URL_GET_BY_ID).json(cond).send();
    let msg = response.unwrap().text().unwrap();
    if msg.eq(r#"{"Ok":null}"#) {
        return None;
    }
    match serde_json::from_str::<Result<Instance>>(&msg).unwrap() {
        Ok(x) => Some(x),
        Err(_) => None
    }
}

pub fn send_business_object<T>(meta_key: &str, bo: &T) -> Result<u64> where T: Serialize {
    send_business_object_with_sys_context(meta_key, bo, &HashMap::new())
}

pub fn send_business_object_with_sys_context<T>(meta_key: &str, bo: &T, sys_context: &HashMap<String, String>) -> Result<u64> where T: Serialize {
    let mut instance = Instance::new(meta_key).unwrap();
    instance.content = serde_json::to_string(bo).unwrap();
    instance.sys_context = sys_context.clone();

    let response = CLIENT.post(URL_INPUT).json(&instance).send();
    let id_s: String = response.unwrap().text().unwrap();
    if id_s.contains("Err") {
        return Err(NatureError::VerifyError(id_s));
    }
    serde_json::from_str(&id_s)?
}

pub fn get_instance_by_id(id: u64, meta_full: &str) -> Option<Instance> {
    get_state_instance_by_id(id, meta_full, 0)
}

pub fn get_state_instance_by_id(id: u64, meta_full: &str, sta_ver: i32) -> Option<Instance> {
    info!("get state instance by id {}", &id);
    let para = InsCond::new(id, meta_full, "", sta_ver);
    let response = CLIENT.post(URL_GET_BY_ID).json(&para).send();
    let msg = response.unwrap().text().unwrap();
    if msg.eq(r#"{"Ok":null}"#) {
        return None;
    }
    match serde_json::from_str::<Result<Instance>>(&msg).unwrap() {
        Ok(x) => Some(x),
        Err(_) => None
    }
}

pub fn wait_for_order_state(order_id: u64, state_ver: i32) -> Instance {
    loop {
        if let Some(ins) = get_state_instance_by_id(order_id, "B:sale/orderState:1", state_ver) {
            return ins;
        } else {
            warn!("not found state instance, will retry");
            sleep(Duration::from_nanos(3000000))
        }
    }
    // panic!("can't find order and state");
}

pub fn send_with_context<T>(meta_key: &str, bo: &T, context: &HashMap<String, String>) -> Result<u64> where T: Serialize {
    let mut instance = Instance::new(meta_key).unwrap();
    instance.content = serde_json::to_string(bo).unwrap();
    instance.context = context.clone();

    let response = CLIENT.post(URL_INPUT).json(&instance).send();
    let id_s: String = response.unwrap().text().unwrap();
    if id_s.contains("Err") {
        return Err(NatureError::VerifyError(id_s));
    }
    serde_json::from_str(&id_s)?
}

pub fn get_by_key(id: u64, meta: &str, para: &str, sta_version: i32) -> Option<Instance> {
    let para = InsCond {
        id: id,
        time_ge: None,
        time_lt: None,
        other: NoIdCond {
            meta: meta.to_string(),
            key_gt: "".to_string(),
            key_ge: "".to_string(),
            key_lt: "".to_string(),
            key_le: "".to_string(),
            para: para.to_string(),
            state_version: sta_version,
            limit: 11,
        }
    };
    get_by_id(&para)
}

pub fn loop_get_by_key(id: u64, meta: &str, para: &str, sta_version: i32) -> Instance {
    loop {
        if let Some(ins) = get_by_key(id, meta, para, sta_version) {
            return ins;
        } else {
            warn!("not found state instance, will retry");
            sleep(Duration::from_nanos(3000000))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn order_id_test() {
        let rtn = get_instance_by_id(1, "B:sale/order:1");
        dbg!(rtn);
    }

    #[test]
    #[ignore]
    fn order_state_test() {
        let rtn = get_state_instance_by_id(1, "B:sale/orderState:1", 1);
        dbg!(rtn);
    }
}
