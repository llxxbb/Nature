extern crate rand;

use lru_time_cache::LruCache;
use self::rand::{Rng, thread_rng};
use std::collections::HashMap;
use std::ops::Range;
use std::ptr;
use std::sync::Mutex;
use std::time::Duration;
use super::*;

lazy_static! {
    static ref CACHE_MAPPING: Mutex<LruCache<Thing, (Vec<Relation>, HashMap<Thing, Range<f32>>)>> = Mutex::new(LruCache::<Thing, (Vec<Relation>, HashMap<Thing, Range<f32>>)>::with_expiry_duration(Duration::from_secs(3600)));

}

pub struct RelationCache;

impl RelationCache {
    pub fn get(from: &Thing) -> Result<Vec<Relation>> {
        debug!("get relation for thing : {:?}", from);
        let (relations, balances) = get_balanced(from)?;
        Ok(weight_filter(&relations, &balances))
    }
}

fn get_balanced(from: &Thing) -> Result<(Vec<Relation>, HashMap<Thing, Range<f32>>)> {
    let mut cache = CACHE_MAPPING.lock().unwrap();
    if let Some(balances) = cache.get(from) {
        debug!("get balances from cache for thing : {:?}", from);
        return Ok(balances.clone());
    }
    debug!("get balances from db for thing : {:?}", from);
    let relations = OneStepFlowDaoImpl::get_relations(from)?;
    let label_groups = get_label_groups(&relations);
    let rtn = (relations, weight_calculate(&label_groups));
    let rtn_clone = rtn.clone();
    cache.insert(from.clone(), rtn);
    Ok(rtn_clone)
}

fn weight_filter(relations: &Vec<Relation>, balances: &HashMap<Thing, Range<f32>>) -> Vec<Relation> {
    let mut rtn: Vec<Relation> = Vec::new();
    let rnd = thread_rng().gen::<f32>();
    for m in relations {
        let _ = match balances.get(&m.to) {
            Some(rng) => if rng.contains(&rnd) {
                rtn.push(m.clone());
            },
            None => rtn.push(m.clone())
        };
    }
    rtn
}

/// weight group will be cached
fn weight_calculate(labels: &HashMap<String, Vec<Relation>>) -> HashMap<Thing, Range<f32>> {
    let mut rtn: HashMap<Thing, Range<f32>> = HashMap::new();
    // calculate "to `Thing`"'s weight
    for (_, group) in labels {
        let sum = group.iter().fold(0u16, |sum, mapping| sum + mapping.weight.proportion as u16);
        if sum <= 0 {
            continue;
        }
        let mut begin = 0.0;
        let last = group.last().unwrap();
        for m in group {
            let w = m.weight.proportion as f32 / sum as f32;
            let end = begin + w;
            if ptr::eq(m, last) {
                // last must great 1
                rtn.insert(m.to.clone(), begin..1.1);
            } else {
                rtn.insert(m.to.clone(), begin..end);
            }
            begin = end;
        }
    }
    rtn
}

/// group by labels
fn get_label_groups(maps: &Vec<Relation>) -> HashMap<String, Vec<Relation>> {
// labels as key, value : Mappings have same label
    let mut labels: HashMap<String, Vec<Relation>> = HashMap::new();
    for mapping in maps {
        let label = mapping.weight.label.clone();
        if label.is_empty() {
            continue;
        }
        let mappings = labels.entry(label).or_insert(Vec::new());
        mappings.push(mapping.clone());
    }
    labels
}

