use super::*;
use rand::{Rng, thread_rng};


#[test]
fn test_random(){
    let x: f32 = thread_rng().gen();
    println!("the random : {}", x);

}