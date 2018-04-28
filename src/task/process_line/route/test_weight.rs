use std::ops::Range;
use super::*;

#[test]
fn test_range() {
    let values = (0.0..1.1);
    assert!(values.contains(&0.0));
    assert!(values.contains(&0.5));
    assert!(values.contains(&1.0));
    assert!(!values.contains(&6.0));
    assert!(!values.contains(&-0.1));
}