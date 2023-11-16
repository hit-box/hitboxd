use hitbox_tower::{
    configuration::serializers::method,
    configuration::{RequestExtractor, RequestPredicate, ResponsePredicate},
    Method, StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Predicates {
    pub request: Vec<RequestPredicate>,
    pub response: Vec<ResponsePredicate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub name: String,
    pub path: String,
    #[serde(with = "method")]
    pub method: Method,
    pub key: Vec<RequestExtractor>,
    pub predicates: Predicates,
    pub backend: String,
    pub upstream: String,
}

impl Default for Predicates {
    fn default() -> Self {
        Self {
            request: Vec::new(),
            response: vec![ResponsePredicate::StatusCode {
                code: StatusCode::OK,
            }],
        }
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        let default_extractors = vec![
            RequestExtractor::Method,
            RequestExtractor::Path {
                path: String::from("/{path}*"),
            },
        ];
        Self {
            name: String::from("all"),
            path: String::from("/{path}*"),
            method: Method::GET,
            key: default_extractors,
            predicates: Predicates::default(),
            backend: String::new(),
            upstream: String::new(),
        }
    }
}
