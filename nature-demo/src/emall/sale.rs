use std::thread::sleep;
use std::time::Duration;

use crate::{get_state_instance_by_id, send_business_object};
use crate::entry::{Commodity, Order, SelectedCommodity};

pub fn send_order_to_nature() -> String {
    // create an order
    let order = create_order_object();
    let id = send_business_object("/sale/order", &order).unwrap();

    // send again
    let id2 = send_business_object("/sale/order", &order).unwrap();
    assert_eq!(id2, id);

    // check created instance for order state
    wait_until_order_state_is_ready(&id)
}

fn wait_until_order_state_is_ready(order_id: &str) -> String {
    loop {
        if let Some(ins) = get_state_instance_by_id(order_id, "B:sale/orderState:1", 1) {
            assert_eq!(ins.id, order_id);
            assert_eq!(ins.states.contains("new"), true);
            let from = ins.from.as_ref().unwrap();
            assert_eq!(from.meta, "B:sale/order:1");
            return ins.id;
        } else {
            sleep(Duration::from_nanos(200000))
        }
    }
}

fn create_order_object() -> Order {
    Order {
        user_id: 123,
        price: 1000,
        items: vec![
            SelectedCommodity {
                item: Commodity { id: 1, name: "phone".to_string(), price: 800 },
                num: 1,
            },
            SelectedCommodity {
                item: Commodity { id: 2, name: "battery".to_string(), price: 100 },
                num: 2,
            }
        ],
        address: "a.b.c".to_string(),
    }
}
