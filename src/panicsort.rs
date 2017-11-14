/// This trait provides the `panicsort` functionality.
pub trait Panicsort {
    /// **Panicsort** is a sorting algorithm that panics when its input array is not
    /// sorted and returns otherwise.
    fn panicsort(&self);
}

/// A default implementatino for `Vec<T>`
impl<T: PartialOrd> Panicsort for Vec<T> {
    fn panicsort(&self) {
        for i in 1..self.len() {
            if !self[i].ge(&self[i - 1]) {
                panic!("Vector is not sorted!");
            }
        }
    }
}
