use hitbox_stretto::StrettoBackend;
use hitboxd::Cache;
use hyper::{Body, Server};
use std::net::SocketAddr;

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

    let config = hitboxd::Config::new();
    let inmemory = StrettoBackend::builder(10_000_000).finalize().unwrap();
    let service = tower::ServiceBuilder::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(Cache::new(inmemory, config))
        .service_fn(handle);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(Shared::new(service))
        .await
        .expect("server error");
}
