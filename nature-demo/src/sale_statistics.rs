use std::thread::sleep;
use std::time::Duration;

use crate::entry::{Commodity, Order, SelectedCommodity};

use crate::send_business_object;

#[test]
fn sale_statistics_test() {
    // create an order
    let order = order_1();
    let _id = send_business_object("/sale/order", &order).unwrap();
    // simulate sum more then once.
    sleep(Duration::from_secs(2));
    let order = order_2();
    let _id = send_business_object("/sale/order", &order).unwrap();
    let order = order_3();
    let _id = send_business_object("/sale/order", &order).unwrap();
}


fn order_1() -> Order {
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

fn order_2() -> Order {
    Order {
        user_id: 124,
        price: 305,
        items: vec![
            SelectedCommodity {
                item: Commodity { id: 3, name: "cup".to_string(), price: 5 },
                num: 1,
            },
            SelectedCommodity {
                item: Commodity { id: 2, name: "battery".to_string(), price: 100 },
                num: 3,
            }
        ],
        address: "a.b.c".to_string(),
    }
}

fn order_3() -> Order {
    Order {
        user_id: 125,
        price: 7006,
        items: vec![
            SelectedCommodity {
                item: Commodity { id: 1, name: "phone".to_string(), price: 700 },
                num: 10,
            },
            SelectedCommodity {
                item: Commodity { id: 3, name: "cup".to_string(), price: 6 },
                num: 1,
            }
        ],
        address: "a.b.c".to_string(),
    }
}

