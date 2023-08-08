use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use hitbox_backend::predicates::Predicate;
use hitbox_http::CacheableHttpRequest;
use http::method::Method;
use hyper::Body;

pub type BoxPredicate = Box<dyn Predicate<Subject = CacheableHttpRequest<Body>> + Send + Sync>;

pub struct Config {
    pub endpoints: HashMap<String, Endpoint<BoxPredicate>>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            endpoints: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Endpoint<P> {
    pub name: String,
    pub path: String,
    pub methods: Vec<Method>,
    pub request_predicate: Arc<P>,
}
