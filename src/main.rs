extern crate failure;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;

use futures::prelude::*;

use hyper::{Post, StatusCode, header::ContentLength};
use hyper::server::{Http, Request, Response, Service};

use serde_json::Value;

struct CSGOMute;

impl Service for CSGOMute {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Post, "/") => Box::new(req.body().concat2().map(|b| {
                let bad_request: &[u8] = b"Missing Field";

                match serde_json::from_slice::<Value>(b.as_ref()) {
                    Ok(json) => {
                        if let Err(_) = parse_json(json) {
                            return Response::new()
                                .with_status(StatusCode::BadRequest)
                                .with_body(bad_request)
                                .with_header(ContentLength(bad_request.len() as u64));
                        }

                        Response::new().with_status(StatusCode::Ok)
                    }
                    Err(_) => Response::new()
                        .with_status(StatusCode::BadRequest)
                        .with_body(bad_request)
                        .with_header(ContentLength(bad_request.len() as u64)),
                }
            })),
            _ => Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}

fn parse_json(json: Value) -> Result<(), failure::Error> {
    

    Ok(())
}

fn main() {
    let addr = "127.0.0.1:37967".parse().unwrap();

    let server = Http::new().bind(&addr, || Ok(CSGOMute)).unwrap();
    println!("Listening on http://{}", server.local_addr().unwrap());
    server.run().unwrap();
}
