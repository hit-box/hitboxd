use crate::predicate::Predicate;
use http::{HeaderMap, HeaderValue, Request, Response};

pub(crate) struct Query {
    inner: Vec<(String, String)>,
}

impl Query {
    pub(crate) fn new(inner: Vec<(String, String)>) -> Self {
        Self { inner }
    }
}

impl<T> Predicate<Request<T>> for Query {
    fn predicate(&self, source: &Request<T>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::Headers;
    use crate::path::Path;
    use crate::status_code::StatusCode;
}
