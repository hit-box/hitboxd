use std::task::{Context, Poll};

use futures_util::future;
use hyper::service::Service;
use hyper::{Body, Request, Response, Server};

#[derive(Debug)]
pub struct CacheService {
    endpoints: &'static Vec<String>,
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
        future::ok(rsp)
    }
}

pub struct ServiceWrapper {
    endpoints: Vec<String>,
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
            endpoints: &self.endpoints,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let addr = "127.0.0.1:1337".parse().unwrap();
    let server = Server::bind(&addr).serve(ServiceWrapper {
        endpoints: vec![String::from("index")],
    });
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
