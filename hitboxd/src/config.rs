use crate::external_configuration;
use hitbox::backend::CacheBackend;
use hitbox::policy::PolicyConfig;
use hitbox_http::SerializableHttpResponse;
use std::collections::HashMap;
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

    pub fn from_external(config: external_configuration::Config) -> Self {
        let backends: HashMap<String, Arc<dyn CacheBackend<SerializableHttpResponse>>> = config
            .backends
            .into_iter()
            .map(|b| (b.name(), b.initialize()))
            .collect();
        let upstreams: HashMap<String, Vec<crate::Upstream>> = config
            .upstreams
            .into_iter()
            .map(|u| {
                (
                    u.name,
                    u.addresses.into_iter().map(crate::Upstream::from).collect(),
                )
            })
            .collect();
        let endpoints = config
            .endpoints
            .into_iter()
            .map(|source_endpoint| crate::Endpoint {
                name: source_endpoint.name,
                routing: crate::endpoint::Routing {
                    path_pattern: source_endpoint.path,
                    methods: vec![source_endpoint.method],
                },
                backend: backends.get(&source_endpoint.backend).unwrap().clone(),
                upstreams: upstreams.get(&source_endpoint.upstream).unwrap().clone(),
                request_predicates: source_endpoint.predicates.request,
                response_predicates: source_endpoint.predicates.response,
                extractors: source_endpoint.key,
                policy: PolicyConfig::default(),
            })
            .collect();
        Config { endpoints }
    }
}
