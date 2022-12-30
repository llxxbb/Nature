use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::common::Result;

#[inline]
pub fn generate_id<T: Hash>(value: &T) -> Result<u64> {
    let mut s = DefaultHasher::new();
    value.hash(&mut s);
    Ok(s.finish())
}
