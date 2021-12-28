use http::Request;
use crate::predicate::Predicate;

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
