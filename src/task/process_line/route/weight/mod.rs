extern crate rand;

use self::rand::{Rng, thread_rng};
use std::ops::Range;
use std::ptr;
use super::*;

pub fn get_relations(from: &Thing) -> Result<Vec<Mapping>> {
    let (relations, balances) = get_balanced(from)?;
    Ok(weight_filter(&relations, &balances))
}

fn get_balanced(from: &Thing) -> Result<(Vec<Mapping>, HashMap<Thing, Range<f32>>)> {
    let mut cache = MAPPING_CACHE.lock().unwrap();
    if let Some(balances) = cache.get(from) {
        return Ok(balances.clone());
    }
    let relations = MappingDaoService::get_relations(from)?;
    let label_groups = get_label_groups(&relations);
    let rtn = (relations, weight_calculate(&label_groups));
    let rtn_clone = rtn.clone();
    cache.insert(from.clone(), rtn);
    Ok(rtn_clone)
}

fn weight_filter(relations: &Vec<Mapping>, balances: &HashMap<Thing, Range<f32>>) -> Vec<Mapping> {
    let mut rtn: Vec<Mapping> = Vec::new();
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
fn weight_calculate(labels: &HashMap<String, Vec<Mapping>>) -> HashMap<Thing, Range<f32>> {
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
fn get_label_groups(maps: &Vec<Mapping>) -> HashMap<String, Vec<Mapping>> {
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
    labels
}

#[cfg(test)]
mod test_weight;