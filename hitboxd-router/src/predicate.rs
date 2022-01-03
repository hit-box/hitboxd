use http::{Request, Response};

pub(crate) trait Predicate<T> {
    fn predicate(&self, source: &T) -> bool;
}
