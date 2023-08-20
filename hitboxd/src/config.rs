use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use hitbox::predicate::Predicate;
use hitbox::Extractor;
use hitbox_http::CacheableHttpRequest;
use http::method::Method;
use hyper::Body;

pub type BoxPredicate = Box<dyn Predicate<Subject = CacheableHttpRequest<Body>> + Send + Sync>;
pub type BoxExtractor = Box<dyn Extractor<Subject = CacheableHttpRequest<Body>> + Send + Sync>;

pub struct Config {
    pub endpoints: HashMap<String, Endpoint<BoxPredicate, BoxExtractor>>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            endpoints: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Endpoint<P, E> {
    pub name: String,
    pub path: String,
    pub methods: Vec<Method>,
    pub request_predicate: Arc<P>,
    pub extractors: Arc<E>,
}
