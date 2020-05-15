use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::http::{StatusCode};
mod routes;
pub use crate::routes::{index, about};

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match _req.uri().path() {
        "/" => Ok(index(_req)),
        "/about" => Ok(about(_req)),
        _ => Ok(Response::builder()
                .status(StatusCode::from_u16(404).unwrap())
                .body("Page not found".into())
                .unwrap()
            )
    }
    
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
