extern crate slowsort;

use slowsort::*;

#[test]
fn sorts_vectors() {
    let mut unsorted_vec = vec![1, 4, 6, 7, 9, 2, 3, 8, 5];

    unsorted_vec.slowsort();

    assert_eq!(unsorted_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
