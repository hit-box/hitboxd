use async_trait::async_trait;
use hitbox::{dev::CacheBackend, CachedValue};
use hitbox_backend::{
    response2::CacheableResponse,
    serializer::{JsonSerializer, Serializer},
    BackendError, BackendResult, DeleteStatus,
};
use hitbox_tower::Cache;
use stretto::AsyncCache;

#[derive(Clone)]
pub struct InMemoryBackend {
    cache: AsyncCache<String, Vec<u8>>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self {
            cache: AsyncCache::new(12960, 1e6 as i64, tokio::spawn).unwrap(),
        }
    }
}

#[async_trait]
impl CacheBackend for InMemoryBackend {
    async fn get<T>(&self, key: String) -> BackendResult<Option<CachedValue<T::Cached>>>
    where
        T: CacheableResponse,
        <T as CacheableResponse>::Cached: serde::de::DeserializeOwned,
    {
        match self.cache.get(&key).await {
            Some(cached) => Ok(Some(
                JsonSerializer::<Vec<u8>>::deserialize(cached.value().to_owned())
                    .map_err(BackendError::from)
                    .unwrap(),
            )),
            None => Ok(None),
        }
    }

    async fn set<T>(
        &self,
        key: String,
        value: CachedValue<T::Cached>,
        ttl: Option<u32>,
    ) -> BackendResult<()>
    where
        T: CacheableResponse + Send,
        <T as CacheableResponse>::Cached: serde::Serialize + Send,
    {
        let serialized =
            JsonSerializer::<Vec<u8>>::serialize(&value).map_err(BackendError::from)?;
        self.cache.insert(key, serialized, 42).await;
        Ok(())
    }

    async fn delete(&self, key: String) -> BackendResult<DeleteStatus> {
        unimplemented!()
    }

    async fn start(&self) -> BackendResult<()> {
        Ok(())
    }
}
