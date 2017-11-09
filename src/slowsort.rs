pub trait Slowsort {
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
