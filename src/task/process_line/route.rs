use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;
use super::*;

pub type RouteTask = Route<DeliveryService>;
pub struct Route<T> {
    delivery_service: PhantomData<T>
}

impl<T> Route<T> where T: DeliveryTrait {
    pub fn do_route_task(carrier: Carrier<StoreInfo>) {
        let instance = &carrier.content.data.instance.clone();
        if let Ok(relations) = RelationCache::get(&instance.data.thing) {
            // no relations
            if relations.len() == 0 {
                let _ = TableDelivery::delete(&carrier.id);
                return;
            }
            let relations = Self::filter_relations(instance, relations);
            Self::delivery_relations(carrier, instance, relations);
        };
    }

    fn filter_relations(instance: &Instance, maps: Vec<Relation>) -> Vec<Relation> {
        let mut rtn: Vec<Relation> = Vec::new();
        for m in maps {
            if !Self::context_check(&instance.data.context, &m) {
                continue;
            }
            if !Self::status_check(&instance.data.status, &m) {
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
        let biz = route.instance.thing.key.clone();
        match T::create_and_finish_carrier(route, carrier, biz, DataType::Route as u8) {
            Ok(new) => send_carrier(CHANNEL_DISPATCH.sender.lock().unwrap().clone(), new),
            Err(_) => ()
        }
    }
}

