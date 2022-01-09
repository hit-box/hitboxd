use crate::cache::Cache;
use crate::configuration::Configuration;
use crate::endpoint::Endpoint as ConfigurationEndpoint;
use hitboxd_endpoint::endpoint::HttpEndpoint;

impl From<ConfigurationEndpoint<Cache>> for HttpEndpoint {
    fn from(source: ConfigurationEndpoint<Cache>) -> Self {
        let status_codes = source
            .response
            .map(|v| v.status_codes)
            .flatten()
            .unwrap_or_default();
        Self::http(source.path, status_codes)
    }
}

impl From<Configuration<Cache>> for Vec<HttpEndpoint> {
    fn from(config: Configuration<Cache>) -> Self {
        config
            .endpoints
            .into_iter()
            .map(HttpEndpoint::from)
            .collect()
    }
}
