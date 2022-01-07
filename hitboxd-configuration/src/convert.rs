use crate::cache::Cache;
use crate::configuration::Configuration;
use hitboxd_handler::handler::Handler;
use crate::endpoint::Endpoint;

impl From<Endpoint<Cache>> for Handler {
    fn from(_: Endpoint<Cache>) -> Self {
        todo!()
    }
}

impl From<Configuration<Cache>> for Vec<Handler> {
    fn from(config: Configuration<Cache>) -> Self {
        config.endpoints
            .into_iter()
            .map(Handler::from)
            .collect()
    }
}
