use nature::domain::{ConverterParameter, ConverterReturned, Instance, Result};

use crate::entry::Order;

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn order_to_item(para: &ConverterParameter) -> ConverterReturned {
    dbg!(&para.from.content);
    let order: Order = match serde_json::from_str(&para.from.content) {
        Ok(ord) => ord,
        Err(e) => {
            dbg!(&e);
            return ConverterReturned::LogicalError { msg: e.to_string() };
        }
    };
    let money = "B:sale/item/money:1";
    let count = "B:sale/item/count:1";
    let mut content: Vec<(String, String, u64)> = vec![];
    let oid = format!("/{}", para.from.id);
    for one in order.items {
        let para = one.item.id.to_string() + &oid;
        content.push((money.to_string(), para.to_string(), one.num as u64 * one.item.price));
        content.push((count.to_string(), para, one.num as u64));
    }

    let rtn: Vec<Instance> = content.iter().map(|one| {
        let mut ins = Instance::default();
        ins.para = one.1.to_string();
        ins.meta = one.0.to_string();
        ins.content = one.2.to_string();
        ins
    }).collect();

    ConverterReturned::Instances { ins: rtn }
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes_definitions)]
pub extern fn order2item(para: &Instance) -> Result<Instance> {
    let order: Order = serde_json::from_str(&para.content)?;
    let mut content: Vec<(String, u64)> = vec![];
    for one in order.items {
        let id = one.item.id;
        let money_key = id.to_string() + &"/money".to_string();
        let count_key = id.to_string() + &"/count".to_string();
        content.push((money_key, one.num as u64 * one.item.price));
        content.push((count_key, one.num as u64));
    }
    let mut rtn = para.clone();
    rtn.content = serde_json::to_string(&content)?;
    Ok(rtn)
}