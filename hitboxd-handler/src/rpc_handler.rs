use crate::predicate::Predicate;
use http::{Request, Response};

#[derive(Debug)]
pub struct RpcHandler;

impl<T> Predicate<Request<T>> for RpcHandler {
    fn predicate(&self, source: &Request<T>) -> bool {
        todo!()
    }
}

impl<T> Predicate<Response<T>> for RpcHandler {
    fn predicate(&self, source: &Response<T>) -> bool {
        todo!()
    }
}