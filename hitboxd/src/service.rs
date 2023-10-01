use hitbox_stretto::StrettoBackend;
use hitbox_tower::{CacheConfig, EndpointConfig};

use std::{fmt::Debug, sync::Arc};

use hitbox::fsm::CacheFuture;
use hitbox_http::{CacheableHttpRequest, CacheableHttpResponse, FromBytes};
use http::{Request, Response};
use hyper::body::{Body, HttpBody};
use tower::Service;

use hitbox_tower::future::Transformer;

pub struct CacheService<S> {
    upstream: S,
    //backends: Arc<HashMap<String, Arc<B>>>,
    config: Arc<crate::Config>,
}

impl<S> CacheService<S> {
    pub fn new(
        upstream: S,
        //backends: Arc<HashMap<String, Arc<B>>>,
        config: Arc<crate::Config>,
    ) -> Self {
        CacheService {
            upstream,
            //backends,
            config,
        }
    }
}

impl<S> Clone for CacheService<S>
where
    S: Clone,
    //B: Clone,
{
    fn clone(&self) -> Self {
        Self {
            upstream: self.upstream.clone(),
            //backends: Arc::clone(&self.backends),
            config: Arc::clone(&self.config),
        }
    }
}

impl<S, ResBody> Service<Request<Body>> for CacheService<S>
where
    S: Service<Request<Body>, Response = Response<ResBody>> + Clone + Send + 'static,
    //B: CacheBackend + Clone + Send + Sync + 'static,
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
        StrettoBackend,
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
        let endpoint = self.config.endpoints.first().unwrap().clone();
        //let endpoint = self
        //.config
        //.endpoints
        ////.values()
        //.into_iter()
        //.find(|endpoint| {
        //ResourceDef::new(endpoint.path.as_str()).is_match(req.uri().path())
        //&& endpoint.methods.contains(req.method())
        //})
        //.unwrap();
        let transformer = Transformer::new(self.upstream.clone());
        let configuration = EndpointConfig {
            request_predicates: endpoint.request_predicates,
            response_predicates: endpoint.response_predicates,
            extractors: endpoint.extractors,
            policy: endpoint.policy,
        };
        CacheFuture::new(
            endpoint.backend,
            CacheableHttpRequest::from_request(req),
            transformer,
            Arc::new(configuration.request_predicates()),
            Arc::new(configuration.response_predicates()),
            Arc::new(configuration.extractors()),
            Arc::new(configuration.policy()),
        )
    }
}
