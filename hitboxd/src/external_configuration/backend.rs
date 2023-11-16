use hitbox::backend::CacheBackend;
use hitbox_http::SerializableHttpResponse;
use hitbox_redis::RedisBackend;
use hitbox_stretto::StrettoBackend;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct InMemory {
    pub name: String,
    pub capacity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Redis {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    InMemory(InMemory),
    Redis(Redis),
}

impl Backend {
    pub fn initialize(self) -> Arc<dyn CacheBackend<SerializableHttpResponse>> {
        match self {
            Backend::InMemory(backend) => Arc::new(
                StrettoBackend::builder(backend.capacity as i64)
                    .finalize()
                    .unwrap(),
            ),
            Backend::Redis(_backend) => Arc::new(RedisBackend::builder().build().unwrap()),
        }
    }
    pub fn name(&self) -> String {
        match self {
            Backend::InMemory(backend) => backend.name.clone(),
            Backend::Redis(backend) => backend.name.clone(),
        }
    }
}

impl Default for InMemory {
    fn default() -> Self {
        Self {
            name: String::from("StrettoBackend"),
            capacity: 10_000_000,
        }
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::InMemory(InMemory::default())
    }
}
