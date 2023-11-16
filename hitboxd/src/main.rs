use hitboxd::{Cache, Config};
use hyper::http::{Request, Response};
use hyper::{Body, Server};
use std::fs;
use std::{net::SocketAddr, sync::Arc};
use tower::make::Shared;

async fn handle(mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let ext = req.extensions().get::<Vec<hitboxd::Upstream>>().unwrap();
    let upstream = ext.first().unwrap();
    let client: hyper::Client<hyper::client::HttpConnector, hyper::Body> =
        hyper::client::Client::builder().build_http();
    let mut parts = req.uri().clone().into_parts();
    let address = format!("{}:{}", upstream.host, upstream.port);
    parts.authority = Some(hyper::http::uri::Authority::try_from(address).unwrap());
    parts.scheme = Some(hyper::http::uri::Scheme::HTTP);
    let uri = hyper::http::uri::Uri::from_parts(parts).unwrap();
    *req.uri_mut() = uri;
    let response = client.request(req).await;
    dbg!(&response);
    response
}

#[tokio::main]
async fn main() {
    let file_path = String::from("config_hitbox.yaml");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let external_config =
        serde_yaml::from_str::<hitboxd::external_configuration::Config>(&contents).unwrap();

    let subscriber = tracing_subscriber::fmt().pretty().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let config = Config::from_external(external_config);
    let config = Arc::new(config);
    let cache_layer = Cache { config };
    let service = tower::ServiceBuilder::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(cache_layer)
        .service_fn(handle);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(Shared::new(service))
        .await
        .expect("server error");
}
