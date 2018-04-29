pub use self::weight::*;
use std::collections::HashMap;
use std::collections::HashSet;
use super::*;

pub fn do_route(carrier: Carrier<StoreInfo>) {
    let instance = &carrier.data.0.clone();
    if let Ok(relations) = get_relations(&instance.data.thing) {
        // no relations
        if relations.len() == 0 {
            let _ = CarrierDaoService::delete(&carrier.id);
            return;
        }
        let relations = filter_relations(instance, relations);
        delivery_relations(carrier, instance, relations);
    };
}

fn filter_relations(instance: &Instance, maps: Vec<Mapping>) -> Vec<Mapping> {
    let mut rtn: Vec<Mapping> = Vec::new();
    for m in maps {
        if !context_check(&instance.data.context, &m) {
            continue;
        }
        if !status_check(&instance.data.status, &m) {
            continue;
        }
        rtn.push(m);
    }
    rtn
}


fn context_check(_contexts: &HashMap<String, String>, _mapping: &Mapping) -> bool {
    // TODO
    false
}

fn status_check(_status: &HashSet<String>, _mapping: &Mapping) -> bool {
    // TODO
    false
}

fn delivery_relations(carrier: Carrier<StoreInfo>, instance: &Instance, maps: Vec<Mapping>) {
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

mod weight;

#[cfg(test)]
mod test_route;

