use define::*;
use instance::*;
use nature::*;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use thing::*;
use uuid::{NAMESPACE_DNS, Uuid, UuidBytes};

#[derive(Debug, Default)]
struct MyWorldConnectionService {
    input_counter: AtomicUsize,
}

impl Nature for MyWorldConnectionService {
    fn flow(&self, _data: Instance) -> Result<UuidBytes> {
        self.input_counter.fetch_add(1, Ordering::SeqCst);
        Ok(*Uuid::new_v3(&NAMESPACE_DNS, "hello").as_bytes())
    }
}


#[test]
fn flow() {
//    lazy_static! {
//        static ref MOCK : MyWorldConnectionService  = MyWorldConnectionService::default();
//    }
//    let svc = MyWorldConnectionService::default();
    let rocket = super::start_rocket_server(&NatureService);
    let client = Client::new(rocket).expect("valid rocket instance");
    let json = ::serde_json::to_string(&(
        Instance {
            id: *Uuid::new_v3(&NAMESPACE_DNS, "hello").as_bytes(),
            data: InstanceNoID {
                thing: Thing {
                    key: String::from("test"),
                    version: 1,
                },
                execute_time: 0,
                create_time: 0,
                content: String::from("hello"),
                context: String::new(),
            },
        })).unwrap();
//    let json = r#"
//        {
//            "thing":{
//                "id":"test",
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
    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Ok":[11,172,237,228,64,20,63,157,183,32,23,63,104,161,201,51]}"#);
}