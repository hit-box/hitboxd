use crate::Config;

use std::sync::Arc;

use tower::Layer;

use crate::CacheService;

#[derive(Clone)]
pub struct Cache {
    pub config: Arc<Config>,
}

impl Cache {
    pub fn new(config: crate::Config) -> Cache {
        Cache {
            config: Arc::new(config),
        }
    }
}

impl<S> Layer<S> for Cache {
    type Service = CacheService<S>;

    fn layer(&self, upstream: S) -> Self::Service {
        CacheService::new(upstream, Arc::clone(&self.config))
    }
}
