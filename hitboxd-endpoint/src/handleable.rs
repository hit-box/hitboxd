use crate::cacheable::Cacheable;
use crate::predicate::Predicate;
use http::{Request, Response};

pub trait Handleable<T>:
    Sync + Send + Cacheable + Predicate<Request<T>> + Predicate<Response<T>>
{
}
