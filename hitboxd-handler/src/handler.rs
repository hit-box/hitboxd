use crate::predicate::Predicate;
use http::{Request, Response};
use crate::http_handler::HttpHandler;
use crate::rpc_handler::RpcHandler;

#[derive(Debug)]
pub enum Handler {
    HttpHandler(HttpHandler),
    RpcHandler(RpcHandler),
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
