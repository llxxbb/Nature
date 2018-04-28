use super::*;

#[test]
fn test_random(){
    let x: f32 = thread_rng().gen();
    println!("the random : {}", x);

}