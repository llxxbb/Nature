extern crate futures;
///! rpc server, collect data from different rpc client then call the server
extern crate hyper;

use biz::WorldConnectionInput;
use self::futures::{future, Stream};
use self::futures::future::Future;
use self::hyper::{Error, Method, StatusCode};
use self::hyper::Chunk;
use self::hyper::header::ContentLength;
use self::hyper::server::{Http, Request, Response, Service};
use service::SERVICE;
use std::result::Result::*;
use super::serde_json;


struct WebServer;

pub fn start_web_server(port: &str) {
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    let closure = move || Ok(WebServer);
    let web_server = Http::new().bind(&addr, closure).unwrap();
    info!("##### Web server started at : {} ---------------------------", port);
    web_server.run().unwrap();
}

impl Service for WebServer {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Post, "/input") => {
                Box::new(req.body().concat2().map(|b| {
                    match handle_input_para(b) {
                        Ok(input) => {
                            let rtn = SERVICE.input(input);
                            match rtn {
                                Ok(sn) => {
                                    return Response::new()
                                        .with_status(StatusCode::Ok)
                                        .with_body(sn.to_string());
                                }
                                Err(msg) => {
                                    return Response::new()
                                        .with_status(StatusCode::UnprocessableEntity)
                                        .with_header(ContentLength(msg.len() as u64))
                                        .with_body(msg);
                                }
                            }
                        }
                        Err(msg) => {
                            return Response::new()
                                .with_status(StatusCode::UnprocessableEntity)
                                .with_header(ContentLength(msg.len() as u64))
                                .with_body(msg);
                        }
                    }
                }
                ))
            }
            _ => {
                Box::new(future::ok(Response::new().with_status(StatusCode::NotFound)))
            }
        }
    }
}

fn handle_input_para(chunk: Chunk) -> Result<WorldConnectionInput, String> {
    let json: WorldConnectionInput = if let Ok(j) = serde_json::from_slice(chunk.as_ref()) {
        j
    } else {
        return Err(String::from("Incorrect json for [WorldConnectionInput]"));
    };
    if json.define.biz.is_empty() {
        return Err(String::from("[biz] must not be empty!"));
    }
    Ok(json)
}