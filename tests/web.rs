extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future, Stream};
use self::hyper::client::Client;
use self::hyper::{Method, Request, StatusCode};
use self::hyper::header::{ContentLength, ContentType};
use self::tokio_core::reactor::Core;
use std::io::*;
use std::str;

#[test]
fn it_works() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure().keep_alive(true).build(&handle);

    let json = r#"{"name":"lxb","number":10}"#;
    let url = "http://127.0.0.1:3000/input".parse().unwrap();
    let mut req = Request::new(Method::Post, url);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);

    println!("test begin ----------------");
    let post = client.request(req).and_then(|res| {
        assert!(res.status() == StatusCode::Ok);
        res.body().concat2()
    });

    let posted = core.run(post).unwrap();
    println!("POST: {}", str::from_utf8(&posted).unwrap());
    println!("test end ----------------");

    drop(client);
}
