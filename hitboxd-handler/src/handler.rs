use crate::predicate::Predicate;
use http::{Request, Response};
use crate::http_handler::{HttpHandler, HandlerRequest, HandlerResponse};
use crate::rpc_handler::RpcHandler;
use crate::path::Path;
use crate::status_code::StatusCode;

#[derive(Debug)]
pub enum Handler {
    HttpHandler(HttpHandler),
    RpcHandler(RpcHandler),
}

impl Handler {
    pub fn http(path: String, status_codes: Option<Vec<u16>>) -> Handler {
        Handler::HttpHandler(HttpHandler {
            request: HandlerRequest { path: Path::new(path) },
            response: HandlerResponse { status_codes: status_codes.map(StatusCode::new) },
            cache: 0
        })
    }
}

impl<T> Predicate<Request<T>> for Handler {
    fn predicate(&self, source: &Request<T>) -> bool {
        match self {
            Handler::HttpHandler(http_handler) => http_handler.predicate(source),
            Handler::RpcHandler(_) => unreachable!(),
        }
    }
}

impl<T> Predicate<Response<T>> for Handler {
    fn predicate(&self, source: &Response<T>) -> bool {
        match self {
            Handler::HttpHandler(http_handler) => http_handler.predicate(source),
            Handler::RpcHandler(_) => unreachable!(),
        }
    }
}
