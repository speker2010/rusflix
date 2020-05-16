use hyper::{Body, Request, Response};
use crate::views::index::index as indexView;
use crate::views::about::about as aboutView;
use crate::layouts::main::main;

pub fn index<B>(_req: Request<B>) -> Response<Body> {
    let index = indexView();
    let title: String = "Rustflix home".into();
    let text = main(title, index);
    Response::new(text.to_string().into())
}

pub fn about<B>(_req: Request<B>) -> Response<Body> {
    let about = aboutView();
    let title = "About Rusflix".into();
    let text = main(title, about);
    Response::new(text.to_string().into())
}