extern crate nature;
extern crate rocket;
extern crate serde_json;

use self::nature::data::Instance;
use self::rocket::http::ContentType;
use server_starter::*;

mod server_starter;


#[test]
fn submit_single() {
    println!("submit_single test---------------");
    // prepare input para
    let json = serde_json::to_string(&(
        Instance::default())).unwrap();

    // call service
    let client = get_client();
    let mut response = client.post("/input")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    // verify returned
    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Ok":[11,172,237,228,64,20,63,157,183,32,23,63,104,161,201,51]}"#);
}