extern crate rand;

/// This trait provides the `bogosort` functionality.
pub trait Bogosort {
    /// **Bogosort** or **random sort** is a highly ineffective sorting algorithm based on
    /// the _"generate and test"_ paradigm. The sorting function permutes its input and
    /// then checks whether the generated sequence is correctly ordered.
    ///
    /// If not, a new permutation is generated:
    ///
    /// ```
    /// while not isInOrder(deck):
    ///     shuffle(deck)
    /// ```
    /// The permutations can either be enumerated in order to avoid duplicates or
    /// completely randomized.
    fn bogosort(&mut self);
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

/// The trait implementation for the `Vec<T>` type is based on the randomized approach
/// that randomly permutes the function input.
impl<T: PartialOrd> Bogosort for Vec<T> {
    fn bogosort(&mut self) {
        while !is_sorted(self) {
            let mut new_vec: Vec<T> = Vec::new();
            while self.len() > 0 {
                let length = self.len();
                new_vec.push(self.remove(rand::random::<usize>() % length));
            }
            self.append(&mut new_vec);
        }
    }
}
