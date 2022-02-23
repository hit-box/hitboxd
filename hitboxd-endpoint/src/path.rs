use crate::predicate::Predicate;
use actix_router::ResourceDef;
use http::Request;

#[derive(Debug)]
pub(crate) struct Path {
    pattern: ResourceDef,
}

impl Path {
    pub(crate) fn new(path: String) -> Self {
        Self {
            pattern: ResourceDef::new(path),
        }
    }
}

impl<T> Predicate<Request<T>> for Path {
    fn predicate(&self, source: &Request<T>) -> bool {
        self.pattern.is_match(source.uri().path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::path::Path;

    #[test]
    fn test_request_path() {
        let cases = vec![
            ("/path/to/resource/", "/path/to/resource/", true),
            ("/resource/", "/path/to/resource/", false),
            ("/path/to/resource/", "/path/{arg1}/resource/", true),
            ("/res/42", "/res/{arg1}", true),
            ("/res/42/res2/name", "/res/{arg1}/res2/{arg2}", true),
            ("/res/42/res/name", "/res/{arg1}/res2/{arg2}", false),
            ("/res/42/res2/name", "/res/{arg1}", false),
            ("/res/42/res2/name", r"/res/{arg1:\d+}", false),
            ("/res/42", r"/res/{arg1:\d+}", true),
            ("/res/42", r"/res/{arg1:\D+}", false),
            ("/res/42/res2", r"/res/{arg1:\d+}", false),
            ("/r/42/r2/name", r"/r/{arg1:\d+}/r2/{arg2:\D+}", true),
            ("/r/name/r2/42", r"/r/{arg1:\d+}/r2/{arg2:\D+}", false),
        ];

        for (path, exp, result) in cases {
            let request = Request::builder().uri(path).body(()).unwrap();
            let path = Path::new(exp.to_string());

            assert_eq!(result, path.predicate(&request));
        }
    }
}
