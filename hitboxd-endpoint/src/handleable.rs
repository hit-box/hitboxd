use crate::cacheable::Cacheable;
use crate::predicate::Predicate;
use http::{Request, Response};

pub trait Handleable<T>: Sync + Send + Cacheable
{
    fn request(&self, req: &Request<T>) -> bool;
    fn response(&self, res: &Response<T>) -> bool;
    fn upstream(&self) -> String;
}
