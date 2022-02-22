use crate::predicate::Predicate;
use actix_router::ResourceDef;
use http::Request;
use regex::Regex;

#[derive(Debug)]
enum Pattern {
    Regex(Regex),
    Actix(ResourceDef),
}

#[derive(Debug)]
pub(crate) struct Path {
    pattern: Pattern,
}

impl Path {
    pub(crate) fn new(path: String) -> Self {
        let pattern = if path.starts_with("^") {
            Pattern::Regex(Regex::new(&path).unwrap())
        } else {
            Pattern::Actix(ResourceDef::new(path))
        };

        Self { pattern }
    }
}

impl<T> Predicate<Request<T>> for Path {
    fn predicate(&self, source: &Request<T>) -> bool {
        let path = source.uri().path();

        match &self.pattern {
            Pattern::Regex(p) => p.is_match(path),
            Pattern::Actix(p) => p.is_match(path),
        }
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
            ("/res/42/res2/name", r"^/res/\d+/res\d/\w+?", true),
            ("/res/42/res/name", r"^/res/\d+/res\d/\w+?", false),
        ];

        for (path, exp, result) in cases {
            dbg!(path, exp, result);

            let request = Request::builder().uri(path).body(()).unwrap();
            let path = Path::new(exp.to_string());

            assert!(path.predicate(&request) == result);
        }
    }
}
