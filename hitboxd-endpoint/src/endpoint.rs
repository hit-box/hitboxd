use crate::http_handler::{HandlerRequest, HandlerResponse, HttpHandler};
use crate::path::Path;
use crate::predicate::Predicate;
use crate::rpc_handler::RpcHandler;
use crate::status_code::StatusCode;
use http::{Request, Response};

#[derive(Debug)]
pub enum Endpoint {
    Http(HttpHandler),
    Rpc(RpcHandler),
}

impl Endpoint {
    pub fn http(path: String, status_codes: Option<Vec<u16>>) -> Endpoint {
        Endpoint::Http(HttpHandler {
            request: HandlerRequest {
                path: Path::new(path),
            },
            response: HandlerResponse {
                status_codes: status_codes.map(StatusCode::new),
            },
            cache: 0,
        })
    }
}

impl<T> Predicate<Request<T>> for Endpoint {
    fn predicate(&self, source: &Request<T>) -> bool {
        match self {
            Endpoint::Http(http_handler) => http_handler.predicate(source),
            Endpoint::Rpc(_) => unreachable!(),
        }
    }
}

impl<T> Predicate<Response<T>> for Endpoint {
    fn predicate(&self, source: &Response<T>) -> bool {
        match self {
            Endpoint::Http(http_handler) => http_handler.predicate(source),
            Endpoint::Rpc(_) => unreachable!(),
        }
    }
}
