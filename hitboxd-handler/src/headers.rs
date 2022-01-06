use crate::predicate::Predicate;
use http::{HeaderMap, HeaderValue, Request, Response};

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
        has_headers(self, source.headers())
    }
}

impl<T> Predicate<Response<T>> for Headers {
    fn predicate(&self, source: &Response<T>) -> bool {
        has_headers(self, source.headers())
    }
}

fn has_headers(headers: &Headers, head: &HeaderMap<HeaderValue>) -> bool {
    let matched_headers = headers
        .inner
        .iter()
        .map(|(key, value)| {
            head.get(key).map(
                |found| found.to_str().map(|v| v.cmp(value)), // log error `ToStrError`
            )
        })
        .flatten()
        .flatten()
        .filter(|v| v.is_eq());
    matched_headers.count() == headers.inner.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::Headers;
    
    

    #[test]
    fn test_response_headers_matched() {
        let response = Request::builder()
            .header("X-Foo-One", "Bar")
            .header("X-Foo-Two", "Bar")
            .body(())
            .unwrap();
        let headers = Headers::new(vec![
            (String::from("X-Foo-One"), String::from("Bar")),
            (String::from("X-Foo-Two"), String::from("Bar")),
        ]);
        assert!(headers.predicate(&response));
    }
    #[test]
    fn test_response_headers_missed() {
        let request = Request::builder()
            .header("X-Foo-One", "Bar")
            .body(())
            .unwrap();
        let headers = Headers::new(vec![
            (String::from("X-Foo-One"), String::from("Bar")),
            (String::from("X-Foo-Two"), String::from("Bar")),
        ]);
        assert!(!headers.predicate(&request));
    }
}
