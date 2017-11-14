/// This trait provides the `slowsort` functionality.
pub trait Slowsort {
    /// **Slowsort** is a humurous sorting algorithm based on the principle of _multiply
    /// and surrender_. The algorithm itself is of recursive nature. It finds the maximum
    /// of the sorted array, places that maximum at the end and sorts the remaining array
    /// recursively.
    ///
    /// A detailed explanation of Slowsort can be found on [Wikipedia](https://en.wikipedia.org/wiki/Slowsort).
    fn slowsort(&mut self);
}


fn slowsort_step<T: PartialOrd + Copy>(vec: &mut Vec<T>, i: usize, j: usize) {
    if i < j {
        let m: usize = (i + j) / 2;
        slowsort_step(vec, i, m);
        slowsort_step(vec, m + 1, j);

        if vec[j] < vec[m] {
            // switch vec[j] and vec[m]
            let buffer = vec[m];
            vec[m] = vec[j];
            vec[j] = buffer;
        }

        slowsort_step(vec, i, j - 1);
    }
}


impl<T: PartialOrd + Copy> Slowsort for Vec<T> {
    fn slowsort(&mut self) {
        let length = self.len();
        slowsort_step(self, 0, length - 1);
    }
}
