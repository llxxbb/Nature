use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use chrono::prelude::*;

use crate::{get_state_instance_by_id, send_business_object_with_sys_context, wait_for_order_state};
use crate::entry::Payment;

pub fn user_pay(order_id: &str) {
    wait_until_order_account_is_ready(order_id);
    let _first = pay(order_id, 100, "a", Local::now().timestamp_millis());
    dbg!("payed first");
    let time = Local::now().timestamp_millis();
    let _second = pay(order_id, 200, "b", time);
    dbg!("payed second");
    let _third = pay(order_id, 700, "c", Local::now().timestamp_millis());
    dbg!("payed third");
    let _second_repeat = pay(order_id, 200, "b", time);
    dbg!("payed second repeat");
    let _ = wait_for_order_state(order_id, 2);
}

fn wait_until_order_account_is_ready(order_id: &str) {
    loop {
        if let Some(_) = get_state_instance_by_id(order_id, "B:finance/orderAccount:1", 1) {
            break;
        } else {
            sleep(Duration::from_nanos(200000))
        }
    }
}

fn pay(id: &str, num: u32, account: &str, time: i64) -> String {
    let payment = Payment {
        order: id.to_string(),
        from_account: account.to_string(),
        paid: num,
        pay_time: time,
    };
    let mut sys_context: HashMap<String, String> = HashMap::new();
    sys_context.insert("target.id".to_string(), id.to_string());
    match send_business_object_with_sys_context("finance/payment", &payment, &sys_context) {
        Ok(id) => id,
        _ => "0".to_string()
    }
}

