//! This crate provides an implementation of various quality sorting methods.
//! Only the most useless or inefficient sorting algorithms are implemented.
//! You may use them in your production application, altough I would strongly
//! advise against that. Currently, the following sorting algorithms are implemented:
//!
//! ## Panicsort
//!
//! This sorting method kind of follows the principle of _check and surrender_
//! and simply panics when the array or vector is not sorted:
//!
//! ```should_panic
//! # mod sorting;
//! # use sorting::*;
//! let unsorted = vec![5, 7, 8, 2, 1, 0];
//! unsorted.panicsort();   // will panic
//! ```
//!
//! ## Slowsort
//!
//! This sorting algorithm recursively sorts the input array by finding the maximum
//! of the sorted array, placing that maximum at the end and sorting the remaining
//! array.
//!
//! ```
//! # mod sorting;
//! # use sorting::*;
//! let mut unsorted = vec![5, 7, 8, 2, 1, 0];
//! unsorted.slowsort();
//! ```
//!
//! ## Bogosort
//!
//! This highly inefficient algorithm scrambles the input vector until it is
//! sorted. Depending on your luck and the length of the input vector this might
//! never return.
//!
//! ```
//! # mod sorting;
//! # use sorting::*;
//! let mut unsorted = vec![5, 7, 8, 2, 1, 0];
//! unsorted.bogosort();    // might take a while...
//! ```
#![warn(missing_docs)]

mod slowsort;
mod bogosort;
mod panicsort;

pub use slowsort::*;
pub use bogosort::*;
pub use panicsort::*;
