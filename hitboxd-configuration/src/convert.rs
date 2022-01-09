use crate::cache::Cache;
use crate::configuration::Configuration;
use crate::endpoint::Endpoint;
use hitboxd_endpoint::endpoint::Endpoint as DEndpoint;

impl From<Endpoint<Cache>> for DEndpoint {
    fn from(source: Endpoint<Cache>) -> Self {
        let status_codes = source.response.map(|v| v.status_codes).flatten();
        Self::http(source.path, status_codes)
    }
}

impl From<Configuration<Cache>> for Vec<DEndpoint> {
    fn from(config: Configuration<Cache>) -> Self {
        config.endpoints.into_iter().map(DEndpoint::from).collect()
    }
}
