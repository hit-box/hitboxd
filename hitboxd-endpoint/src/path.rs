use crate::predicate::Predicate;
use http::Request;
use regex::Regex;

#[derive(Debug)]
pub(crate) struct Path {
    exp: Regex,
}

impl Path {
    pub(crate) fn new(tpl: String) -> Self {
        let mut tpl = tpl
            .replace("{digit}", r"\d+")
            .replace("{string}", r"\D+")
            .replace("{any}", r".+");
        tpl = format!("^{}?", tpl);

        Self {
            exp: Regex::new(&tpl).unwrap(),
        }
    }
}

impl<T> Predicate<Request<T>> for Path {
    fn predicate(&self, source: &Request<T>) -> bool {
        self.exp.is_match(source.uri().path())
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
            ("/path/to/resource/", "/path/{string}/resource/", true),
            ("/res/42", "/res/{digit}", true),
            ("/res/42/res2/name", "/res/{digit}/res2/{string}", true),
            ("/res/42/res/name", "/res/{digit}/res2/{string}", false),
            ("/res/42/res2/42", "/res/{digit}/res2/{string}", false),
            ("/res/name/res2/name", "/res/{digit}/res2/{string}", false),
            ("/res/42/res2/name", "/res/{any}", true),
            ("/res/42/res2/name", r"/res/\d+/res\d/\w+", true),
            ("/res/42/res/name", r"/res/\d+/res\d/\w+", false),
        ];

        for (path, exp, result) in cases {
            let request = Request::builder().uri(path).body(()).unwrap();
            let path = Path::new(exp.to_string());

            assert!(path.predicate(&request) == result);
        }
    }
}
