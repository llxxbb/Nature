use nature::domain::Instance;

use crate::{loop_get_by_key, send_instance};

#[test]
#[ignore]
fn test() {
    #[derive(Serialize)]
    struct Delivery;

    let mut ins = Instance::new("delivery").unwrap();
    ins.path.para = "A/B".to_string();
    let id = send_instance(&ins).unwrap();

    finish_delivery(id, "A/B", "mid");
    finish_delivery(id, "B/C", "mid");
    finish_delivery(id, "C/D", "end");
}

fn finish_delivery(id: u64, para: &str, context: &str) {
    let _last = loop_get_by_key(id, "B:deliveryState:1", para, 1);
    let mut ins = Instance::new("deliveryState").unwrap();
    ins.path.para = para.to_string();
    ins.states.insert("finished".to_owned());
    ins.context.insert(context.to_string(), context.to_string());
    ins.path.state_version = 2;
    let _id = send_instance(&ins).unwrap();
}
