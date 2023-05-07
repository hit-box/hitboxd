use axum::{
    extract::State,
    http::{uri::Uri, Request, Response},
    routing::any,
    Router,
};
use hitbox_tower::Cache;
use hitboxd::config::read_config;
use hitboxd::inmemory::InMemoryBackend;
use hyper::{client::HttpConnector, Body};
use hyper_tls::HttpsConnector;
use std::net::SocketAddr;
use std::path::Path;
use tower::ServiceBuilder;

type Client = hyper::client::Client<HttpsConnector<HttpConnector>, Body>;

#[derive(Clone)]
struct AppState {
    client: Client,
    server: String,
}

#[tokio::main]
async fn main() {
    let path = Path::new("config.yaml");
    let config = read_config(path).unwrap();

    let https = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let backend = InMemoryBackend::new();
    let server = config.server.to_string();
    let state = AppState { client, server };

    let app = Router::new()
        .route("/*name", any(upstream))
        .with_state(state)
        .layer(ServiceBuilder::new().layer(Cache::builder().backend(backend).build()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn upstream(State(state): State<AppState>, mut req: Request<Body>) -> Response<Body> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    let uri = format!(
        "{server}{path_query}",
        server = state.server,
        path_query = path_query
    );
    *req.uri_mut() = Uri::try_from(uri).unwrap();
    state.client.request(req).await.unwrap()
}
