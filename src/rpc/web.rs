///! rpc server, collect data from different rpc client then call the server
use super::futures::{future, Stream};
use super::futures::future::Future;
use super::hyper::{Error, Method, StatusCode};
use super::hyper::header::ContentLength;
use super::hyper::server::{Http, Request, Response, Service};
use super::serde_json;

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
    type Error = Error;
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
                            .with_body(MISSING);
                    };
                    // Validate the request parameters, returning
                    // early if an invalid input is detected.
                    if json["name"] == serde_json::Value::Null {
                        return Response::new()
                            .with_status(StatusCode::BadRequest)
                            .with_body("name field not found");
                    }

                    let body = format!("Hello {}, your number is {}", json["name"], json["number"]);
                    Response::new()
                        .with_header(ContentLength(body.len() as u64))
                        .with_body(body)
                }))
            }
            _ => {
                Box::new(future::ok(Response::new().with_status(StatusCode::NotFound)))
            }
        }
    }
}
