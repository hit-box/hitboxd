use http::Response;

use crate::predicate::Predicate;
use std::collections::HashSet;

#[derive(Debug)]
pub(crate) struct StatusCodes {
    inner: HashSet<u16>,
}

impl StatusCodes {
    pub(crate) fn new(inner: Vec<u16>) -> Self {
        Self {
            inner: inner.iter().cloned().collect(),
        }
    }
}

impl<T> Predicate<Response<T>> for StatusCodes {
    fn predicate(&self, source: &Response<T>) -> bool {
        let status_code = source.status().as_u16();
        self.inner.contains(&status_code)
    }
}

#[cfg(test)]
mod tests {

    use crate::status_code::StatusCodes;

    use super::*;

    #[test]
    fn test_response_status_cacheable() {
        let response = Response::builder()
            .status(http::StatusCode::OK)
            .body(())
            .unwrap();
        // https://www.rfc-editor.org/rfc/rfc7231#section-6.1
        let status = StatusCodes::new(vec![200, 301, 410]);
        assert!(status.predicate(&response));
    }

    #[test]
    fn test_response_status_not_cacheable() {
        let response = Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(())
            .unwrap();
        // https://www.rfc-editor.org/rfc/rfc7231#section-6.1
        let status = StatusCodes::new(vec![200, 301, 410]);
        assert!(!status.predicate(&response));
    }
}
