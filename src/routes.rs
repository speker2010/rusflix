use hyper::{Body, Request, Response};

pub fn index(_req: Request<Body>) -> Response<Body> {
    Response::new("Welcome to Rusflix".into())
}

pub fn about(_req: Request<Body>) -> Response<Body> {
    Response::new("Rusflix is a streaming service".into())
}