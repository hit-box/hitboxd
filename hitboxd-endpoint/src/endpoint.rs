use crate::cacheable::{CacheError, Cacheable};
use crate::path::Path;
use crate::predicate::Predicate;
use crate::status_code::StatusCodes;
use crate::Handleable;
use http::{Request, Response};

#[derive(Debug)]
pub struct HttpEndpoint {
    pub(crate) request: RequestPredictors,
    pub(crate) response: ResponsePredictors,
    pub(crate) cache: i32,
}

#[derive(Debug)]
pub(crate) struct RequestPredictors {
    pub(crate) path: Path,
}

#[derive(Debug)]
pub(crate) struct ResponsePredictors {
    pub(crate) status_codes: StatusCodes,
}

impl HttpEndpoint {
    pub fn http(path: String, status_codes: Vec<u16>) -> HttpEndpoint {
        Self {
            request: RequestPredictors {
                path: Path::new(path),
            },
            response: ResponsePredictors {
                status_codes: StatusCodes::new(status_codes),
            },
            cache: 0,
        }
    }
}

impl<T> Predicate<Request<T>> for HttpEndpoint {
    fn predicate(&self, source: &Request<T>) -> bool {
        self.request.path.predicate(source)
    }
}

impl<T> Predicate<Response<T>> for HttpEndpoint {
    fn predicate(&self, source: &Response<T>) -> bool {
        self.response.status_codes.predicate(source)
    }
}

impl Cacheable for HttpEndpoint {
    fn cache_key(&self) -> Result<Vec<u8>, CacheError> {
        Ok(Vec::new())
    }
}

impl<T> Handleable<T> for HttpEndpoint {}
