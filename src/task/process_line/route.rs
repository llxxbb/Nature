extern crate rand;

use self::rand::{Rng, thread_rng};
use std::collections::HashMap;
use std::collections::HashSet;
use super::*;

pub fn do_route(carrier: Carrier<StoreInfo>) {
    let instance = &carrier.data.0.clone();
    if let Ok(relations) = MappingDaoService::get_relations(&instance.data.thing) {
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
    let balance = gen_label_groups(&maps);
    for m in maps {
        if !context_check(&instance.data.context, &m) {
            continue;
        }
        if !status_check(&instance.data.status, &m) {
            continue;
        }
        if !weight_check(&m, &balance) {
            continue;
        }
        rtn.push(m);
    }
    rtn
}

/// generate weights
fn gen_label_groups(maps: &Vec<Mapping>) -> HashMap<Thing, f32> {
    let mut rtn: HashMap<Thing, f32> = HashMap::new();
    // labels as key, value : Mappings have same label
    let mut labels: HashMap<String, Vec<Mapping>> = HashMap::new();
    for mapping in maps {
        let label = mapping.weight.label.clone();
        if label.is_empty() {
            continue;
        }
        let mappings = labels.entry(label).or_insert(Vec::new());
        mappings.push(mapping.clone());
    }
    // calculate "to `Thing`"'s weight
    for (_, group) in labels {
        let sum = group.iter().fold(0u16, |sum, mapping| sum + mapping.weight.proportion as u16);
        if sum <= 0 {
            continue;
        }
        for m in group {
            rtn.insert(m.to, m.weight.proportion as f32 / sum as f32);
        }
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

fn weight_check(mapping: &Mapping, balance: &HashMap<Thing, f32>) -> bool {
    match balance.get(&mapping.to) {
        // no balance setting then permit through
        None => true,
        // `thread_rng().gen` will generate a number between 0 and 1.
        Some(weight) => thread_rng().gen::<f32>() * weight >= 0.5
    }
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