use crate::predicate::Predicate;
use http::{Request};

pub(crate) struct Query {
    inner: Vec<(String, String)>,
}

impl Query {
    pub(crate) fn new(inner: Vec<(String, String)>) -> Self {
        Self { inner }
    }
}

impl<T> Predicate<Request<T>> for Query {
    fn predicate(&self, _source: &Request<T>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    
    
    
    
}
