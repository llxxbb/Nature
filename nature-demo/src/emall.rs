use crate::emall::finance::user_pay;
use crate::emall::sale::send_order_to_nature;
use crate::emall::warehouse::outbound;
use crate::wait_for_order_state;

#[test]
fn emall_test() {
    dbg!("generate order");
    let id = send_order_to_nature();
    dbg!("pay for order");
    user_pay(&id);
    dbg!("package and outbound");
    outbound(&id);
    dbg!("delivery");
    let _ = wait_for_order_state(&id, 5);
    dbg!("delay for auto signed");
    let _ = wait_for_order_state(&id, 6);
}


mod finance;
mod sale;
mod warehouse;