mod backend;
mod endpoint;
mod group;
mod policy;
mod upstream;

pub use backend::Backend;
pub use endpoint::Endpoint;
pub use policy::Policy;
pub use upstream::{Address, Scheme, Upstream};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub upstreams: Vec<Upstream>,
    pub backends: Vec<Backend>,
    pub endpoints: Vec<Endpoint>,
}
