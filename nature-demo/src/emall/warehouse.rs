use nature::domain::Instance;

use crate::{send_instance, wait_for_order_state};

pub fn outbound(order_id: &str) {
    // for package
    let last = wait_for_order_state(order_id, 3);
    let mut instance = Instance::new("sale/orderState").unwrap();
    instance.id = last.id.to_string();
    instance.state_version = last.state_version + 1;
    instance.states.clear();
    instance.states.insert("outbound".to_string());
    let rtn = send_instance(&instance);
    assert_eq!(rtn.is_ok(), true);
    // for outbound
    let _ = wait_for_order_state(order_id, 4);
}

