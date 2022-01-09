pub enum CacheError {
    Error,
}

pub trait Cacheable {
    fn cache_key(&self) -> Result<Vec<u8>, CacheError>;
}
