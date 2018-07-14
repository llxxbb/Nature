#![feature(range_contains)]

extern crate rand;

use rand::thread_rng;
use rand::Rng;

#[test]
fn test_range() {
    println!("----------------- test_range --------------------");
    let values = 0.0..1.1;
    assert!(values.contains(&1.0));
    assert!(!values.contains(&6.0));
    assert!(!values.contains(&-0.1));
}

#[test]
fn test_random() {
    println!("----------------- test_random --------------------");
    let x: f32 = thread_rng().gen();
    println!("the random : {}", x);
}