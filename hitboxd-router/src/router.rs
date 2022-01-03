use crate::predicate::Predicate;
use http::{Request, Response};

struct CacheKey {
    path: String,
    query: Vec<String>,
    headers: Vec<String>,
}

struct Matcher {
    path: String,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
}

struct Endpoint {
    cache_key: CacheKey,
    matcher: Matcher,
}

struct Router {
    endpoints: Vec<Endpoint>,
}

impl<T> Predicate<Request<T>> for Router {
    fn predicate(&self, source: &Request<T>) -> bool {
        todo!()
    }
}

impl<T> Predicate<Response<T>> for Router {
    fn predicate(&self, source: &Response<T>) -> bool {
        todo!()
    }
}
