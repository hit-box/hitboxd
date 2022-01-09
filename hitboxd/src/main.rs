use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::task::{Context, Poll};

use std::future::Future;
use std::pin::Pin;

use futures_util::future;
use hyper::service::Service;
use hyper::{Body, Request, Response, Server, Client, Uri};

use hitboxd_configuration::cache::{Cache, OverriddenCache};
use hitboxd_configuration::configuration::Configuration;
use hitboxd_endpoint::predicate::Predicate;
use hitboxd_endpoint::Handleable;


pub struct CacheService {
    endpoints: Arc<Vec<Box<dyn Handleable<Body>>>>,
}

impl Service<Request<Body>> for CacheService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let found_endpoint = self
            .endpoints
            .iter()
            .find(|endpoint| endpoint.request(&req));
        match found_endpoint {
            Some(endpoint) => {
                let host = endpoint.upstream();
                Box::pin(async move {
                    let client = Client::new();
                    let scheme = req.uri().scheme_str().unwrap_or_else(|| "http");
                    let path = req.uri().path();
                    let uri = Uri::builder()
                        .scheme(scheme)
                        .authority(host)
                        .path_and_query(path)
                        .build()
                        .unwrap();
                    client.get(uri).await
                })
            },
            None => {
                Box::pin(async move {
                    let client = Client::new();
                    client.request(req).await
                })
            },
        }
    }
}

pub struct ServiceWrapper {
    endpoints: Arc<Vec<Box<dyn Handleable<Body>>>>,
}

impl<T> Service<T> for ServiceWrapper {
    type Response = CacheService;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(CacheService {
            endpoints: self.endpoints.clone(),
        })
    }
}

fn read_config() -> Configuration<Cache> {
    let path = Path::new("test.yaml");
    let mut path_to_file = env::current_dir().unwrap();
    path_to_file.push(path);
    let mut test_yaml = File::open(&path).unwrap();
    let mut s = String::new();
    let _ = test_yaml.read_to_string(&mut s);
    let res: Configuration<OverriddenCache> = serde_yaml::from_str(s.as_str()).unwrap();
    res.into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let addr = "127.0.0.1:1337".parse().unwrap();
    let config = read_config();
    let endpoints: Vec<hitboxd_endpoint::HttpEndpoint> = config.into();
    let mut handleable: Vec<Box<dyn Handleable<Body>>> = Vec::new();
    for endpoint in endpoints {
        handleable.push(Box::new(endpoint))
    }
    let service = ServiceWrapper {
        endpoints: Arc::new(handleable),
    };
    let server = Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
