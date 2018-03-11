use biz::*;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;


#[derive(Debug, Default)]
struct MyWorldConnectionService {
    input_counter: AtomicUsize,
}

impl WorldConnectionService for MyWorldConnectionService {
    fn input(&self, _data: WorldConnectionInput) -> Result<u64, &str> {
        Ok(self.input_counter.fetch_add(1, Ordering::SeqCst) as u64)
    }

    fn input_batch(&self, _batch: Vec<WorldConnectionInput>) -> Result<u64, &str> {
        unimplemented!()
    }

    fn converter_callback(&self) -> Result<u64, &str> {
        unimplemented!()
    }

    fn query(&self) {
        unimplemented!()
    }
}


#[test]
fn input() {
    lazy_static! {
        static ref MOCK : MyWorldConnectionService  = MyWorldConnectionService::default();
    }

    let rocket = super::start_rocket_server(&*MOCK);
    let client = Client::new(rocket).expect("valid rocket instance");
    let json = ::serde_json::to_string(&(
        WorldConnectionInput {
            define: DataDefineBase {
                biz: String::from("test"),
                version: 1,
            },
            content: String::from("hello"),
            context: String::new(),
        })).unwrap();
//    let json = r#"
//        {
//            "define":{
//                "biz":"test",
//                "version": 1
//            },
//            "content":"hello",
//            "context":""
//        }
//        "#;
    println!("{:?}", json);
    let mut response = client.post("/input")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some(r#"{"Ok":123}"#.into()));
}