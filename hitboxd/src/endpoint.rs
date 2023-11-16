use crate::external_configuration;
use hitbox::backend::CacheBackend;
use hitbox::policy::PolicyConfig;
use hitbox_http::SerializableHttpResponse;
use hitbox_stretto::StrettoBackend;
use hitbox_tower::configuration::{RequestExtractor, RequestPredicate, ResponsePredicate};
use hitbox_tower::EndpointConfig;
use hitbox_tower::Method;
use std::sync::Arc;

#[derive(Clone)]
pub struct Routing {
    pub path_pattern: String,
    pub methods: Vec<Method>,
}

#[derive(Clone)]
pub enum Scheme {
    Http,
    Https,
}

#[derive(Clone)]
pub struct Upstream {
    pub host: String,
    pub port: u16,
    pub scheme: Scheme,
}

impl From<external_configuration::Address> for Upstream {
    fn from(address: external_configuration::Address) -> Self {
        Self {
            host: address.host,
            port: address.port,
            scheme: match address.scheme {
                external_configuration::Scheme::Http => Scheme::Http,
                external_configuration::Scheme::Https => Scheme::Https,
            },
        }
    }
}

#[derive(Clone)]
pub struct Endpoint {
    pub name: String,
    pub routing: Routing,
    pub backend: Arc<dyn CacheBackend<SerializableHttpResponse>>,
    pub upstreams: Vec<Upstream>,
    pub request_predicates: Vec<RequestPredicate>,
    pub response_predicates: Vec<ResponsePredicate>,
    pub extractors: Vec<RequestExtractor>,
    pub policy: PolicyConfig,
}

impl Endpoint {
    pub fn new(backend: Arc<StrettoBackend>) -> Self {
        Self {
            name: String::new(),
            routing: Routing {
                path_pattern: String::new(),
                methods: Vec::new(),
            },
            backend,
            upstreams: Vec::new(),
            request_predicates: Vec::new(),
            response_predicates: Vec::new(),
            extractors: Vec::new(),
            policy: Default::default(),
        }
    }

    pub fn to_endpoint_config(self) -> EndpointConfig {
        EndpointConfig {
            request_predicates: self.request_predicates,
            response_predicates: self.response_predicates,
            extractors: self.extractors,
            policy: self.policy,
        }
    }
}
