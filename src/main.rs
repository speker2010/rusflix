#![recursion_limit="256"]

use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::http::{StatusCode};
use futures_util::future;
use hyper_staticfile::Static;
use std::io::Error as IoError;
use std::path::Path;
mod routes;
mod layouts;
mod views;
use routes::{index, about};


async fn handle_request<B>(_req: Request<B>, static_: Static) -> Result<Response<Body>, IoError> {
    let path = _req.uri().path();
    let path_len = path.len();
    let path_begin: &str = if path_len > 7 {
        &path[0..7]
    } else {
        &path[0..path_len]
    };
    if path_begin != "/public" {
        match _req.uri().path() {
            "/" => Ok(index(_req)),
            "/about" => Ok(about(_req)),
            _ => Ok(Response::builder()
                    .status(StatusCode::from_u16(404).unwrap())
                    .body("Page not found".into())
                    .unwrap()
                )
        }
    } else {
        static_.clone().serve(_req).await
    }
    
}

#[tokio::main]
async fn main() {
    let static_ = Static::new(Path::new("src/static/"));
    

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_| {
        let static_ = static_.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, static_.clone())))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
