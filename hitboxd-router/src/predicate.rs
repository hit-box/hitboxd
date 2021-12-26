use http::{Request, Response};

trait Predicate<T> {
    fn predicate(&self, source: &T) -> bool;
}

struct Path {
    inner: String,
}

impl<T> Predicate<Request<T>> for Path {
    fn predicate(&self, source: &Request<T>) -> bool {
        self.inner == source.uri().path()
    }
}

struct StatusCode {
    inner: u16,
}

impl<T> Predicate<Response<T>> for StatusCode {
    fn predicate(&self, source: &Response<T>) -> bool {
        self.inner == source.status().as_u16()
    }
}

struct Headers {
    inner: Vec<(String, String)>
}

impl<T> Predicate<Response<T>> for Headers {
    fn predicate(&self, source: &Response<T>) -> bool {
        let matched_headers = self.inner
            .iter()
            .map(|(key, value)| source.headers()
                .get(key)
                .map(|found| found
                    .to_str()
                    .map(|v| v.cmp(value)) // log error `ToStrError`
                )
            )
            .flatten()
            .flatten()
            .filter(|v| v.is_eq());
        matched_headers.count() == self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_path() {
        let request = Request::builder()
            .uri("https://example.com/path/to/resource/")
            .body(())
            .unwrap();
        let path = Path { inner: String::from("/path/to/resource/") };
        assert!(path.predicate(&request));
    }

    #[test]
    fn test_response_status() {
        let response = Response::builder()
            .status(http::StatusCode::OK)
            .body(())
            .unwrap();
        let status = StatusCode { inner: 200 };
        assert!(status.predicate(&response));
    }

    #[test]
    fn test_response_headers() {
        let response = Response::builder()
            .header("Foo", "Bar")
            .body(())
            .unwrap();
        let headers = Headers {
            inner: vec![(String::from("Foo"), String::from("Bar"))]
        };
        assert!(headers.predicate(&response));
    }
}
