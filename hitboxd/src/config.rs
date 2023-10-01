use crate::external_configuration;
use hitbox::policy::PolicyConfig;
use hitbox_stretto::StrettoBackend;
use std::sync::Arc;

pub struct Config {
    pub endpoints: Vec<crate::Endpoint>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            endpoints: Vec::new(),
        }
    }

    pub fn from_external(
        config: external_configuration::Config,
        backend: Arc<StrettoBackend>,
    ) -> Self {
        let endpoints = config
            .endpoints
            .into_iter()
            .map(|source_endpoint| crate::Endpoint {
                name: source_endpoint.name,
                routing: crate::endpoint::Routing {
                    path_pattern: source_endpoint.path,
                    methods: vec![source_endpoint.method],
                },
                backend: backend.clone(),
                upstreams: Vec::new(),
                request_predicates: source_endpoint.predicates.request,
                response_predicates: source_endpoint.predicates.response,
                extractors: source_endpoint.key,
                policy: PolicyConfig::default(),
            })
            .collect();
        Config { endpoints }
    }
}
