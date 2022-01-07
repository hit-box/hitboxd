use serde::{Deserialize, Serialize};

use crate::cache::{Cache, OverriddenCache};
use crate::request::Request;
use crate::response::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint<CacheType> {
    #[serde(flatten)]
    pub(crate) cache: CacheType,
    pub(crate) path: String,
    pub(crate) request: Option<Request>,
    pub(crate) response: Option<Response>,
}

impl Endpoint<OverriddenCache> {
    pub(crate) fn merge(&self, cache: &Cache) -> Endpoint<Cache> {
        Endpoint {
            cache: self.cache.merge(cache),
            path: self.path.clone(),
            request: self.request.clone(),
            response: self.response.clone(),
        }
    }
}
