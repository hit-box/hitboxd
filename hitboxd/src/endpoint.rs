use hitbox::policy::PolicyConfig;
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
pub struct Upstream {
    pub address: String,
    pub port: u16,
}

#[derive(Clone)]
pub struct Endpoint {
    pub name: String,
    pub routing: Routing,
    pub backend: Arc<StrettoBackend>,
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
