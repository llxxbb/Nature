use super::*;

pub fn do_dispatch(carrier: Carrier<RouteInfo>) {
    let id = &carrier.id.clone();
    let new_carriers = generate_carriers(carrier);
    if new_carriers.len() == 0 {
        return;
    }
    let to_send = new_carriers.clone();
    // save news
    for n in new_carriers {
        let _ = CarrierDaoService::insert(&n);
    }
    // remove old
    let _ = CarrierDaoService::delete(id);
    // to carry
    for task in to_send {
        send_carrier(CHANNEL_CONVERT.sender.lock().unwrap().clone(), task)
    }
}

fn generate_carriers(carrier: Carrier<RouteInfo>) -> Vec<Carrier<ConverterInfo>> {
    let mut new_carriers: Vec<Carrier<ConverterInfo>> = Vec::new();
    let instance = carrier.instance.clone();
    let maps = carrier.data.maps.clone();
    for c in maps {
        let task = ConverterInfo(instance.clone(), c);
        match Carrier::new(task) {
            Ok(x) => new_carriers.push(x),
            Err(err) => {
                ProcessLine::move_to_err(err, carrier);
                return new_carriers;
            }
        }
    }
    new_carriers
}