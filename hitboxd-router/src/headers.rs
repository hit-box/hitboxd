use http::Request;
use crate::predicate::Predicate;

pub(crate) struct Headers {
    inner: Vec<(String, String)>,
}

impl Headers {
    pub(crate) fn new(inner: Vec<(String, String)>) -> Self {
        Self { inner }
    }
}

impl<T> Predicate<Request<T>> for Headers {
    fn predicate(&self, source: &Request<T>) -> bool {
        let matched_headers = self
            .inner
            .iter()
            .map(|(key, value)| {
                source.headers().get(key).map(
                    |found| found.to_str().map(|v| v.cmp(value)), // log error `ToStrError`
                )
            })
            .flatten()
            .flatten()
            .filter(|v| v.is_eq());
        matched_headers.count() == self.inner.len()
    }
}
