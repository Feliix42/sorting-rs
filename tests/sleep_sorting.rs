extern crate sorting;

use std::time::Instant;
use sorting::*;

#[test]
fn sleep_sort_unsigned() {
    let unsorted: Vec<u8> = vec![5, 0, 1, 4, 7, 5, 9, 6];
    let sorted: Vec<_> = unsorted.sleepsort().collect();

    assert_eq!(sorted, vec![0, 1, 4, 5, 5, 6, 7, 9]);
}

#[test]
fn sleep_sort_signed() {
    let unsorted: Vec<i8> = vec![-120, -110, -96, -128];
    let sorted: Vec<_> = unsorted.sleepsort().collect();

    assert_eq!(sorted, vec![-128, -120, -110, -96]);
}

#[test]
fn sleep_sort_faster() {
    use SleepsortSpeed::*;

    let unsorted = vec![10u8, 5, 0, 3];
    let start = Instant::now();
    let sorted: Vec<_> = unsorted.sleepsort_with_speed(Faster(10)).collect();

    assert!(start.elapsed().as_secs() < 6);
    assert_eq!(sorted, vec![0u8, 3, 5, 10]);
}

#[test]
fn sleep_sort_slower() {
    use SleepsortSpeed::*;

    let unsorted = vec![4u8, 2, 0, 3];
    let start = Instant::now();
    let sorted: Vec<_> = unsorted.sleepsort_with_speed(Slower(2)).collect();

    assert!(start.elapsed().as_secs() >= 8);
    assert_eq!(sorted, vec![0u8, 2, 3, 4]);
}
