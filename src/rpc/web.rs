///! rpc server, collect data from different rpc client then call the server
extern crate futures;
extern crate hyper;

use biz::{WorldConnectionInput, WorldConnectionService};
use self::futures::{future, Stream};
use self::futures::future::Future;
use self::hyper::{Error, Method, StatusCode};
use self::hyper::Chunk;
use self::hyper::header::ContentLength;
use self::hyper::server::{Http, Request, Response, Service};
use std::result::Result::*;
use super::serde_json;

pub fn start_web_server<T: WorldConnectionService>(port: &str, server: &'static T) {
    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    let closure = move || Ok(WebServer { service: server });
    let web_server = Http::new().bind(&addr, closure).unwrap();
    info!("##### Web server started at : {} ---------------------------", port);
    web_server.run().unwrap();
}

struct WebServer {
    service: &'static WorldConnectionService,
}

impl Service for WebServer {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    //                        let rtn = SERVICE.input(input);
    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Post, "/input") => {
                let service = self.service;
                Box::new(req.body().concat2().map(move |b| {
                    match handle_input_para(b) {
                        Ok(input) => {
                            let rtn = service.input(input);
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
                }))
            }
            _ => {
                Box::new(future::ok(Response::new().with_status(StatusCode::NotFound)))
            }
        }
    }
}

fn handle_input_para(chunk: Chunk) -> Result<WorldConnectionInput, String> {
    if let Ok(j) = serde_json::from_slice(chunk.as_ref()) {
        return Ok(j);
    } else {
        return Err(String::from("Incorrect json for [WorldConnectionInput]"));
    }
}