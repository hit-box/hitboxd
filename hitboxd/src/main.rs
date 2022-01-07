
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_util::future;
use hyper::service::Service;
use hyper::{Body, Request, Response, Server};

use hitboxd_handler::handler::Handler;
use hitboxd_handler::predicate::Predicate;
use std::path::Path;
use std::env;
use std::fs::File;
use std::io::Read;
use hitboxd_configuration::configuration::Configuration;
use hitboxd_configuration::cache::{Cache, OverriddenCache};

#[derive(Debug)]
pub struct CacheService {
    inner: Arc<Vec<Handler>>,
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
        let handler = self.inner.iter().find(|handler| handler.predicate(&req));
        dbg!(&handler);
        future::ok(rsp)
    }
}

pub struct ServiceWrapper {
    inner: Arc<Vec<Handler>>,
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
            inner: self.inner.clone(),
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
    let handlers = config.into();
    let service = ServiceWrapper {
        inner: Arc::new(handlers),
    };
    let server = Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
