pub trait Circular<T> {
    fn step(self, t: T) -> Self;
}
