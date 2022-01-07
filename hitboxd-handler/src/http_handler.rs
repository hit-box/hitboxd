use crate::predicate::Predicate;
use http::{Request, Response};

#[derive(Debug)]
pub struct HttpHandler {
    pub(crate) request: HandlerRequest,
    pub(crate) response: HandlerResponse,
    pub(crate) cache: i32,
}

impl HttpHandler {
    pub fn new(path: String, status_code: u16) -> Self {
        Self {
            request: HandlerRequest { path },
            response: HandlerResponse { status_code },
            cache: 42,
        }
    }
}

#[derive(Debug)]
pub(crate) struct HandlerRequest {
    path: String,
}

#[derive(Debug)]
pub(crate) struct HandlerResponse {
    status_code: u16,
}

impl<T> Predicate<Request<T>> for HttpHandler {
    fn predicate(&self, source: &Request<T>) -> bool {
        todo!()
    }
}

impl<T> Predicate<Response<T>> for HttpHandler {
    fn predicate(&self, source: &Response<T>) -> bool {
        todo!()
    }
}

