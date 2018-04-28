use std::ops::Range;
use super::*;
use std::ptr;

pub fn get_relations(from: &Thing) -> Result<Vec<Mapping>> {
    let relations = MappingDaoService::get_relations(from)?;

    let label_groups = get_label_groups(&relations);

    let map = weight_calculate(&label_groups);
    // TODO
    Ok(Vec::new())
}


fn weight_check(mapping: &Mapping, balance: &HashMap<Thing, f32>) -> bool {
    match balance.get(&mapping.to) {
        // no balance setting then permit through
        None => true,
        // `thread_rng().gen` will generate a number between 0 and 1.
        Some(weight) => thread_rng().gen::<f32>() * weight >= 0.5
    }
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
            if ptr::eq(m,last) {
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

