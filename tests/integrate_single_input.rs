extern crate nature;
extern crate rocket;
extern crate serde_json;

use self::nature::data::Instance;
use self::rocket::http::ContentType;
use server_starter::*;

mod server_starter;


#[test]
fn must_input_key() {
    println!("must input_key---------------");
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
    assert_eq!(rtn, r#"{"Err":{"VerifyError":"key length can't be zero"}}"#);
}

