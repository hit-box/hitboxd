pub trait Predicate<T> {
    fn predicate(&self, source: &T) -> bool;
}
