use crate::cache::Cache;
use crate::configuration::Configuration;
use crate::endpoint::Endpoint as ConfigurationEndpoint;
use hitboxd_endpoint::endpoint::Endpoint;

impl From<ConfigurationEndpoint<Cache>> for Endpoint {
    fn from(source: ConfigurationEndpoint<Cache>) -> Self {
        let status_codes = source.response.map(|v| v.status_codes).flatten();
        Self::http(source.path, status_codes)
    }
}

impl From<Configuration<Cache>> for Vec<Endpoint> {
    fn from(config: Configuration<Cache>) -> Self {
        config.endpoints.into_iter().map(Endpoint::from).collect()
    }
}
