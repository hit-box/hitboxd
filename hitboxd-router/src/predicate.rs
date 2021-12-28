use http::{Request, Response};

pub(crate) trait Predicate<T> {
    fn predicate(&self, source: &T) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::Path;
    use crate::status_code::StatusCode;
    use crate::headers::Headers;

    #[test]
    fn test_request_path() {
        let request = Request::builder()
            .uri("https://example.com/path/to/resource/")
            .body(())
            .unwrap();
        let path = Path::new(String::from("/path/to/resource/"));
        assert!(path.predicate(&request));
    }

    #[test]
    fn test_response_status() {
        let response = Response::builder()
            .status(http::StatusCode::OK)
            .body(())
            .unwrap();
        let status = StatusCode::new(200);
        assert!(status.predicate(&response));
    }

    #[test]
    fn test_response_headers_matched() {
        let response = Request::builder()
            .header("X-Foo-One", "Bar")
            .header("X-Foo-Two", "Bar")
            .body(())
            .unwrap();
        let headers = Headers::new(
            vec![
                (String::from("X-Foo-One"), String::from("Bar")),
                (String::from("X-Foo-Two"), String::from("Bar")),
            ],
        );
        assert!(headers.predicate(&response));
    }
    #[test]
    fn test_response_headers_missed() {
        let request = Request::builder()
            .header("X-Foo-One", "Bar")
            .body(())
            .unwrap();
        let headers = Headers::new(
            vec![
                (String::from("X-Foo-One"), String::from("Bar")),
                (String::from("X-Foo-Two"), String::from("Bar")),
            ],
        );
        assert!(!headers.predicate(&request));
    }
}
