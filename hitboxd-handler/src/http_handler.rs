use crate::predicate::Predicate;
use http::{Request, Response};
use crate::path::Path;
use crate::status_code::StatusCode;

#[derive(Debug)]
pub struct HttpHandler {
    pub(crate) request: HandlerRequest,
    pub(crate) response: HandlerResponse,
    pub(crate) cache: i32,
}

#[derive(Debug)]
pub(crate) struct HandlerRequest {
    pub(crate) path: Path,
}

#[derive(Debug)]
pub(crate) struct HandlerResponse {
    pub(crate) status_codes: Option<StatusCode>,
}

impl<T> Predicate<Request<T>> for HttpHandler {
    fn predicate(&self, source: &Request<T>) -> bool {
        self.request.path.predicate(source)
    }
}

impl<T> Predicate<Response<T>> for HttpHandler {
    fn predicate(&self, source: &Response<T>) -> bool {
        todo!()
    }
}

