use crate::predicate::Predicate;
use http::Request;

pub(crate) struct Path {
    inner: String,
}

impl Path {
    pub(crate) fn new(inner: String) -> Self {
        Self { inner }
    }
}

impl<T> Predicate<Request<T>> for Path {
    fn predicate(&self, source: &Request<T>) -> bool {
        self.inner == source.uri().path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use crate::path::Path;
    

    #[test]
    fn test_request_path() {
        let request = Request::builder()
            .uri("https://example.com/path/to/resource/")
            .body(())
            .unwrap();
        let path = Path::new(String::from("/path/to/resource/"));
        assert!(path.predicate(&request));
    }
}
