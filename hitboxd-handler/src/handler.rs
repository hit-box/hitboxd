use crate::predicate::Predicate;


#[derive(Debug)]
pub enum Handler {
    HttpHandler(HttpHandler),
    RpcHandler(RpcHandler),
}

impl<T> Predicate<T> for Handler {
    fn predicate(&self, _source: &T) -> bool {
        match self {
            Handler::HttpHandler(_http_handler) => true,
            Handler::RpcHandler(_rpc_handler) => false,
        }
    }
}

#[derive(Debug)]
struct CacheKey {
    path: String,
    query: Vec<String>,
    headers: Vec<String>,
}

#[derive(Debug)]
struct Matcher {
    path: String,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct HttpHandler {
    cache_key: CacheKey,
    matcher: Matcher,
}

#[derive(Debug)]
pub struct RpcHandler;
