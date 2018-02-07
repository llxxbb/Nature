///! rpc server, collect data from different rpc client then call the server
extern crate hyper;
extern crate futures;
extern crate serde_json;

use self::futures::future::Future;
use self::futures::Stream;
use self::hyper::{Method, StatusCode};
use self::hyper::header::ContentLength;
use self::hyper::server::{Http, Request, Response, Service};

pub fn web_rpc(port: &str) {
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    let web_server = Http::new().bind(&addr, || Ok(WebServer)).unwrap();
    info!("##### Web server started at : {} ---------------------------", port);
    web_server.run().unwrap();
}


pub struct WebServer;

static MISSING: &[u8] = b"Missing field";

impl Service for WebServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Post, "/input") => {
                Box::new(req.body().concat2().map(|b| {
                    let json: serde_json::Value = if let Ok(j) = serde_json::from_slice(b.as_ref()) {
                        j
                    } else {
                        return Response::new()
                            .with_status(StatusCode::BadRequest)
                            .with_header(ContentLength(MISSING.len() as u64))
                            .with_body(MISSING);
                    };
                    // Validate the request parameters, returning
                    // early if an invalid input is detected.

                    let body = format!("Hello {}, your number is {}", json["name"], json["number"]);
                    Response::new()
                        .with_header(ContentLength(body.len() as u64))
                        .with_body(body)
                }))
            }
            _ => {
                Box::new(futures::future::ok(Response::new().with_status(StatusCode::NotFound)))
            }
        }
    }
}
