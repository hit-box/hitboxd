use hitbox::{predicate::Predicate, Extractor};
use hitbox_http::{
    extractors::{method::MethodExtractor, path::PathExtractor, NeutralExtractor},
    predicates::{header::HeaderPredicate, query::QueryPredicate, NeutralRequestPredicate},
    CacheableHttpRequest,
};
use hitbox_redis::RedisBackend;
// use hitbox_stretto::StrettoBackend;
use hitboxd::{
    config::{BoxPredicate, Endpoint},
    Cache,
};
use http::Method;
use hyper::{Body, Server};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use hyper::http::{Request, Response};
use tower::make::Shared;

async fn handle(mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let ext = req.extensions().get::<String>();
    dbg!(ext);
    let client: hyper::Client<hyper::client::HttpConnector, hyper::Body> =
        hyper::client::Client::builder().build_http();
    let mut parts = req.uri().clone().into_parts();
    parts.authority = Some(hyper::http::uri::Authority::from_static("httpbin.org"));
    parts.scheme = Some(hyper::http::uri::Scheme::HTTP);
    let uri = hyper::http::uri::Uri::from_parts(parts).unwrap();
    *req.uri_mut() = uri;
    let response = client.request(req).await;
    dbg!(&response);
    response
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt().pretty().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let mut config = hitboxd::Config::new();
    let test_endpoint = Endpoint {
        name: "test".to_owned(),
        path: "/test/".to_owned(),
        methods: vec![Method::GET],
        request_predicate: Arc::new(Box::new(
            NeutralRequestPredicate::new().query("cache".to_owned(), "true".to_owned()),
        )
            as Box<dyn Predicate<Subject = CacheableHttpRequest<Body>> + Send + Sync>),
        extractors: Arc::new(Box::new(NeutralExtractor::new().method().path("{path}*"))
            as Box<dyn Extractor<Subject = CacheableHttpRequest<Body>> + Send + Sync>),
    };
    let ip_endpoint = Endpoint {
        name: "ip".to_owned(),
        path: "/ip".to_owned(),
        methods: vec![Method::GET],
        request_predicate: Arc::new(Box::new(
            NeutralRequestPredicate::new()
                .query("cache".to_owned(), "true".to_owned())
                .header("x-cache".to_owned(), "enable".to_owned()),
        )
            as Box<dyn Predicate<Subject = CacheableHttpRequest<Body>> + Send + Sync>),
        extractors: Arc::new(Box::new(NeutralExtractor::new().method().path("{path}*"))
            as Box<dyn Extractor<Subject = CacheableHttpRequest<Body>> + Send + Sync>),
    };
    config.endpoints = HashMap::with_capacity(2);
    config.endpoints.insert("test".to_owned(), test_endpoint);
    config.endpoints.insert("ip".to_owned(), ip_endpoint);

    let backend = RedisBackend::builder().build().unwrap();
    // let inmemory = StrettoBackend::builder(10_000_000).finalize().unwrap();
    let service = tower::ServiceBuilder::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(Cache::new(backend, config))
        .service_fn(handle);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(Shared::new(service))
        .await
        .expect("server error");
}
