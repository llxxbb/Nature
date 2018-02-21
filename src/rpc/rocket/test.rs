use biz::*;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;
use super::*;


struct MyService;


impl WorldConnectionService for MyService {
    fn input(&self, data: WorldConnectionInput) -> Result<u64, String> {
        unimplemented!()
    }

    fn input_batch(&self, batch: Vec<WorldConnectionInput>) -> Result<u64, String> {
        unimplemented!()
    }

    fn converter_callback(&self) -> Result<u64, String> {
        unimplemented!()
    }

    fn query(&self) {
        unimplemented!()
    }
}

static SERVICE: MockWorldConnectionService = MockWorldConnectionService::new();

#[test]
fn input() {
//    let mock: &'static WorldConnectionService = &MockWorldConnectionService::new();
    let rocket = super::start_rocket_server(&SERVICE);
    let client = Client::new(rocket).expect("valid rocket instance");
//    let json = ::serde_json::to_string()
//        let json = ::serde_json::to_string(&(
//            WorldConnectionInput {
//                define: DataDefineBase {
//                    biz: String::from("test"),
//                    version: 1
//                },
//                content: String::from("hello"),
//                context: String::new(),
//            })).unwrap();
    let json = r#"
        {
            "define":{
                "biz":"test",
                "version": 1
            },
            "content":"hello",
            "context":""
        }
        "#;
    println!("{:?}", json);
    let mut response = client.post("/input")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"Ok":123}"#.into()));
}