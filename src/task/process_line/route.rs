use super::*;

pub fn do_route(carrier: Carrier<StoreTask>) {
    let instance = &carrier.data.0.clone();
    if let Ok(relations) = MappingDaoService::get_relations(&instance.data.thing) {
        // no relations
        if relations.len() == 0 {
            let _ = CarrierDaoService::delete(&carrier.id);
            return;
        }
        delivery_relations(carrier, instance, relations);
    };
}

fn delivery_relations(carrier: Carrier<StoreTask>, instance: &Instance, maps: Vec<Mapping>) {
    let route = RouteInfo { instance: instance.clone(), maps };
    let _ = match Carrier::new(route) {
        Ok(new_carrier) => {
            // insert new first carrier
            if let Ok(_) = CarrierDaoService::insert(&new_carrier) {
                // then delete old carrier
                if let Ok(_) = CarrierDaoService::delete(&carrier.id) {
                    // carry
                    send_carrier(CHANNEL_DISPATCH.sender.lock().unwrap().clone(), new_carrier);
                };
            };
        }
        Err(err) => ProcessLine::move_to_err(err, carrier)
    };
}