use std::collections::HashMap;
use std::collections::HashSet;
use super::*;

pub fn do_route(carrier: Carrier<StoreInfo>) {
    let instance = &carrier.data.instance.clone();
    if let Ok(relations) = RelationCache::get(&instance.data.thing) {
        // no relations
        if relations.len() == 0 {
            let _ = TableDelivery::delete(&carrier.id);
            return;
        }
        let relations = filter_relations(instance, relations);
        delivery_relations(carrier, instance, relations);
    };
}

fn filter_relations(instance: &Instance, maps: Vec<Relation>) -> Vec<Relation> {
    let mut rtn: Vec<Relation> = Vec::new();
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


fn context_check(contexts: &HashMap<String, String>, mapping: &Relation) -> bool {
    for exclude in &mapping.demand.context_exclude {
        if contexts.contains_key(exclude) {
            return false;
        }
    }
    for include in &mapping.demand.context_include {
        if !contexts.contains_key(include) {
            return false;
        }
    }
    true
}

fn status_check(status: &HashSet<String>, mapping: &Relation) -> bool {
    for exclude in &mapping.demand.source_status_exclude {
        if status.contains(exclude) {
            return false;
        }
    }
    for include in &mapping.demand.source_status_include {
        if !status.contains(include) {
            return false;
        }
    }
    true
}

fn delivery_relations(carrier: Carrier<StoreInfo>, instance: &Instance, maps: Vec<Relation>) {
    let route = RouteInfo { instance: instance.clone(), maps };
    match DeliveryImpl::create_and_finish_carrier(route, carrier) {
        Ok(new) => send_carrier(CHANNEL_DISPATCH.sender.lock().unwrap().clone(), new),
        Err(_) => ()
    }
}


