extern crate sorting;

use sorting::*;

#[test]
#[should_panic]
fn not_sorted() {
    let unsorted_vec = vec![1, 2, 4, 3];
    unsorted_vec.panicsort();
}

#[test]
fn is_sorted() {
    let sorted_vec = vec![1, 2, 3, 4];
    sorted_vec.panicsort();
}
