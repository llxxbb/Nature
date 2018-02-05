///! rpc server, collect data from different rpc client then call the server
extern crate hyper;
extern crate futures;

use self::futures::future::Future;

use self::hyper::header::ContentLength;
use self::hyper::server::{Request, Response, Service};

pub struct WebServer;

const PHRASE: &'static str = "Hello, World!";

impl Service for WebServer{

    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        match req {
            _ => {}
        }
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE)
        ))
    }
}
