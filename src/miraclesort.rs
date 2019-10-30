/// This trait provides the `miraclesort` functionality.
pub trait Miraclesort {
    /// Miracle sort is a sort that truly requires a miracle. We keep checking
    /// the array until it is sorted. It requires that some external force (a
    /// miracle?) changes some bits in the computer in a way that it becomes
    /// sorted.
    fn miraclesort(&self);
}

/// This is a mere helper function to determine, whether a vector of objects is sorted.
fn is_sorted<T: PartialOrd>(vec: &Vec<T>) -> bool {
    for i in 1..vec.len() {
        if !vec[i].ge(&vec[i - 1]) {
            return false;
        }
    }

    true
}

/// A default implementatino for `Vec<T>`
impl<T: PartialOrd> Miraclesort for Vec<T> {
    fn miraclesort(&self) {
        while !is_sorted(self) {
            // wait for a miracle to happen...
        }
    }
}
