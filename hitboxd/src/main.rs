use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_util::future;
use hyper::service::Service;
use hyper::{Body, Request, Response, Server};

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
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let rsp = Response::builder();
        let body = Body::from(Vec::from(&b"heyo!"[..]));
        let rsp = rsp.status(200).body(body).unwrap();
        let found_endpoint = self
            .endpoints
            .iter()
            .find(|endpoint| endpoint.predicate(&req));
        match found_endpoint {
            Some(endpoint) => {
                let _cache_key = endpoint.cache_key();
                future::ok(rsp)
            }
            None => {
                // let client = Client::new();
                // let uri = "http://httpbin.org/ip".parse().unwrap_or_default();
                future::ok(rsp)
                // async {
                //     client.get(uri).await
                // }
            }
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
