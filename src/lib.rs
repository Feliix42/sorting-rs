//! This crate provides an implementation of various quality sorting methods.
#![warn(missing_docs)]

mod slowsort;
mod bogosort;
mod panicsort;

pub use slowsort::*;
pub use bogosort::*;
pub use panicsort::*;
