use data::*;
use rocket::http::ContentType;
use rocket::local::Client;

#[test]
fn input_test() {
    println!("----------------- input_test --------------------");
    let rocket = super::start_rocket_server();
    let client = Client::new(rocket).expect("valid rocket instance");
    let json = ::serde_json::to_string(&(
        Instance::default())).unwrap();
    let mut response = client.post("/input")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Ok":[11,172,237,228,64,20,63,157,183,32,23,63,104,161,201,51]}"#);
}

#[test]
fn callback_test(){
    println!("----------------- callback_test --------------------");
    let rocket = super::start_rocket_server();
    let client = Client::new(rocket).expect("valid rocket instance");
    let json = ::serde_json::to_string(&(
        DelayedInstances::default())).unwrap();
    let mut response = client.post("/callback")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Ok":null}"#);
}

#[test]
fn serial_batch_test(){
    println!("----------------- serial_batch_test --------------------");
    let rocket = super::start_rocket_server();
    let client = Client::new(rocket).expect("valid rocket instance");
    let json = ::serde_json::to_string(&(
        SerialBatchInstance::default())).unwrap();
    let mut response = client.post("/serial_batch")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Ok":null}"#);
}

#[test]
fn parallel_batch_test(){
    println!("----------------- parallel_batch_test --------------------");
    let rocket = super::start_rocket_server();
    let client = Client::new(rocket).expect("valid rocket instance");
    let json = ::serde_json::to_string(&(
        ParallelBatchInstance::default())).unwrap();
    let mut response = client.post("/parallel_batch")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Ok":null}"#);
}