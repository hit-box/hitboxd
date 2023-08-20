use std::collections::HashMap;
use std::sync::Arc;

use tower::Layer;

use crate::CacheService;

#[derive(Clone)]
pub struct Cache<B> {
    pub backends: Arc<HashMap<String, Arc<B>>>,
    pub config: Arc<crate::Config>,
}

impl<B> Cache<B> {
    pub fn new(backend: B, config: crate::Config) -> Cache<B> {
        let backends = vec![(String::from("InMemory"), Arc::new(backend))];
        let backends = HashMap::<_, _, std::collections::hash_map::RandomState>::from_iter(
            backends.into_iter(),
        );
        Cache {
            backends: Arc::new(backends),
            config: Arc::new(config),
        }
    }
}

impl<S, B> Layer<S> for Cache<B> {
    type Service = CacheService<S, B>;

    fn layer(&self, upstream: S) -> Self::Service {
        CacheService::new(
            upstream,
            Arc::clone(&self.backends),
            Arc::clone(&self.config),
        )
    }
}
