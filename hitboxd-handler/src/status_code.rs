use http::Response;

use crate::predicate::Predicate;

#[derive(Debug)]
pub(crate) struct StatusCode {
    inner: Vec<u16>,
}

impl StatusCode {
    pub(crate) fn new(inner: Vec<u16>) -> Self {
        Self { inner }
    }
}

impl<T> Predicate<Response<T>> for StatusCode {
    fn predicate(&self, source: &Response<T>) -> bool {
        self.inner
            .iter()
            .any(|status_code| status_code == &source.status().as_u16())
    }
}

#[cfg(test)]
mod tests {
    
    
    use crate::status_code::StatusCode;

    use super::*;

    #[test]
    fn test_response_status() {
        let response = Response::builder()
            .status(http::StatusCode::OK)
            .body(())
            .unwrap();
        let status = StatusCode::new(200);
        assert!(status.predicate(&response));
    }
}
