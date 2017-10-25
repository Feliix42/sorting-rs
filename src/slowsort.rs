pub trait Slowsort {
    fn slowsort(&mut self);
}


impl<T: Ord> Slowsort for Vec<T> {
    fn slowsort(&mut self) {
        unimplemented!();
    }
}
