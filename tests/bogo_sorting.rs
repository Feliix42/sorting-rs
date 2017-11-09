extern crate sorting;

use sorting::*;

#[test]
fn sorts_correctly() {
    let mut unsorted_vec = vec![1, 2, 4, 3, 23, 6, 12, 0];
    unsorted_vec.bogosort();

    assert_eq!(unsorted_vec, vec![0, 1, 2, 3, 4, 6, 12, 23]);
}
