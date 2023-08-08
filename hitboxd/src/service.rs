use std::collections::HashMap;
use std::{fmt::Debug, marker::PhantomData, pin::Pin, sync::Arc};

use actix_router::ResourceDef;
use bytes::Bytes;
use chrono::{Duration, Utc};
use futures::{
    future::{BoxFuture, Map},
    Future, FutureExt,
};
use hitbox::{
    backend::{BackendError, CacheBackend},
    fsm::{CacheFuture, Transform},
    Cacheable, CachedValue,
};
use hitbox_backend::CacheableResponse;
use hitbox_http::{
    predicates::{query::QueryPredicate, NeutralPredicate, NeutralResponsePredicate},
    CacheableHttpRequest, CacheableHttpResponse, FromBytes, SerializableHttpResponse,
};
use http::{Request, Response};
use hyper::body::{Body, HttpBody};
use serde::{de::DeserializeOwned, Serialize};
use tower::Service;

use hitbox::fsm::CacheFuture3;
use tracing::log::warn;

use hitbox_tower::future::{Transformer, UpstreamFuture};

pub struct CacheService<S, B> {
    upstream: S,
    backends: Arc<HashMap<String, Arc<B>>>,
    config: Arc<crate::Config>,
}

impl<S, B> CacheService<S, B> {
    pub fn new(
        upstream: S,
        backends: Arc<HashMap<String, Arc<B>>>,
        config: Arc<crate::Config>,
    ) -> Self {
        CacheService {
            upstream,
            backends,
            config,
        }
    }
}

impl<S, B> Clone for CacheService<S, B>
where
    S: Clone,
    B: Clone,
{
    fn clone(&self) -> Self {
        Self {
            upstream: self.upstream.clone(),
            backends: Arc::clone(&self.backends),
            config: Arc::clone(&self.config),
        }
    }
}

impl<S, B, ResBody> Service<Request<Body>> for CacheService<S, B>
where
    S: Service<Request<Body>, Response = Response<ResBody>> + Clone + Send + 'static,
    B: CacheBackend + Clone + Send + Sync + 'static,
    S::Future: Send,

    // debug bounds
    Body: From<Body>,
    ResBody: FromBytes + HttpBody + Send + 'static,
    ResBody::Error: Debug,
    ResBody::Data: Send,
{
    type Response = Response<ResBody>;
    type Error = S::Error;
    type Future = CacheFuture<
        B,
        CacheableHttpRequest<Body>,
        CacheableHttpResponse<ResBody>,
        Transformer<S, Body>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.upstream.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let endpoint = self
            .config
            .endpoints
            .values()
            .find(|endpoint| {
                ResourceDef::new(endpoint.path.as_str()).is_match(req.uri().path())
                    && endpoint.methods.contains(req.method())
            })
            .unwrap();
        let request_predicate = Arc::clone(&endpoint.request_predicate);
        let transformer = Transformer::new(self.upstream.clone());
        let backend = self.backends.get("InMemory").unwrap();
        CacheFuture::new(
            backend.clone(),
            CacheableHttpRequest::from_request(req),
            transformer,
            request_predicate,
            // Arc::new(NeutralPredicate::new().query("cache".to_owned(), "true".to_owned())),
            Arc::new(NeutralResponsePredicate::new()),
        )
    }
}
