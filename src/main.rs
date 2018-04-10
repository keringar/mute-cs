extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate failure;
extern crate serde;
extern crate serde_json;

use actix_web::{middleware, Application, HttpRequest, HttpResponse, Method, server::HttpServer};

use std::cell::Cell;
use std::sync::Arc;

struct AppState {
    counter: Arc<Cell<usize>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            counter: Arc::new(Cell::new(0)),
        }
    }
}

fn index(req: HttpRequest<AppState>) -> HttpResponse {
    req.state().counter.set(req.state().counter.get() + 1);

    HttpResponse::Ok()
        .body(format!("Number: {}", req.state().counter.get()))
        .unwrap()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        Application::with_state(AppState::new())
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(Method::GET).f(index))
    }).bind("127.0.0.1:37967")
        .expect("Could not bind to 127.0.0.1:37967")
        .run();
}
