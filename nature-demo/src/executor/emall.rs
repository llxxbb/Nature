use chrono::Local;

use nature::domain::{ConverterParameter, ConverterReturned, Instance};

use crate::entry::{Order, OrderAccount, OrderAccountReason, Payment};

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn order_receivable(para: &ConverterParameter) -> ConverterReturned {
    let result = serde_json::from_str(&para.from.content);
    if result.is_err() {
        let msg = format!("generate order receivable error: {}, data: {}", result.err().unwrap(), para.from.content);
        dbg!(&msg);
        return ConverterReturned::LogicalError { msg };
    }
    let order: Order = result.unwrap();
    let oa = OrderAccount {
        receivable: order.price,
        total_paid: 0,
        last_paid: 0,
        reason: OrderAccountReason::NewOrder,
        diff: 0 - order.price as i32,
    };
    let mut instance = Instance::default();
    instance.content = serde_json::to_string(&oa).unwrap();
    ConverterReturned::Instances { ins: vec![instance] }
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn pay_count(para: &ConverterParameter) -> ConverterReturned {
    let result = serde_json::from_str(&para.from.content);
    if result.is_err() {
        dbg!(&para.from.content);
        return ConverterReturned::LogicalError { msg: "unknown content".to_string() };
    }
    let payment: Payment = result.unwrap();
    if para.last_state.is_none() {
        return ConverterReturned::EnvError { msg: "can't find last status instance".to_string() };
    }
    let old = para.last_state.as_ref().unwrap();
    let mut oa: OrderAccount = serde_json::from_str(&old.content).unwrap();
    let mut state = String::new();
    if payment.paid > 0 {
        state = "partial".to_string();
    }
    oa.total_paid += payment.paid;
    oa.diff = oa.total_paid as i32 - oa.receivable as i32;
    if oa.diff > 0 {
        oa.total_paid = oa.receivable;
    }
    if oa.diff == 0 {
        state = "paid".to_string();
    }
    oa.last_paid = payment.paid;
    oa.reason = OrderAccountReason::Pay;
    let mut instance = Instance::default();
    instance.content = serde_json::to_string(&oa).unwrap();
    instance.states.insert(state);
    ConverterReturned::Instances { ins: vec![instance] }
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn stock_out_application(para: &ConverterParameter) -> ConverterReturned {
    // Tn real application need to convert order to store_out_application.
    // but in this demo, we need not do anything.
    dbg!(&para.master.as_ref().unwrap().path.meta);
    ConverterReturned::None
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn go_express(para: &ConverterParameter) -> ConverterReturned {
    // "any one" will be correct by Nature after returned
    let mut ins = Instance::new("any one").unwrap();
    // ... some code to get express info from warehouse system,
    // the follow line simulate the express company name and the waybill id returned
    ins.path.para = "/ems/".to_owned() + &format!("{}", para.from.id);
    // return the waybill
    ConverterReturned::Instances { ins: vec![ins] }
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn auto_sign(_para: &ConverterParameter) -> ConverterReturned {
    // "any one" will be correct by Nature after returned
    let mut ins = Instance::new("any one").unwrap();
    ins.content = format!("type=auto,time={}", Local::now());
    // return the waybill
    ConverterReturned::Instances { ins: vec![ins] }
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn multi_delivery(para: &ConverterParameter) -> ConverterReturned {
    let para: &str = &para.from.path.para;
    let mut ins = Instance::new("abc").unwrap();
    ins.path.para = match para {
        "A/B" => "B/C".to_string(),
        "B/C" => "C/D".to_string(),
        "C/D" => "error".to_string(),
        _ => "err2".to_string()
    };
    // return the waybill
    ConverterReturned::Instances { ins: vec![ins] }
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn multi_warehouse(para: &ConverterParameter) -> ConverterReturned {
    let mut ins = Instance::new_with_null_meta();
    ins.content = para.from.content.to_string();
    // return the waybill
    ConverterReturned::Instances { ins: vec![ins] }
}

