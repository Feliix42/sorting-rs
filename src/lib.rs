pub trait Slowsort {
    fn slowsort(&mut self) -> Self;
}


impl<T: Ord> Slowsort for Vec<T> {
    fn slowsort(&mut self) -> Vec<T> {
        unimplemented!();
    }
}
