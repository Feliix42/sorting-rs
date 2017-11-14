# The world's best and finest sorting algorithms, implemented in Rust

[![Build Status](https://travis-ci.org/Feliix42/sorting-rs.svg?branch=master)](https://travis-ci.org/Feliix42/sorting-rs) [![docs.rs](https://docs.rs/sorting/badge.svg)](https://docs.rs/sorting)

This crate only implements the most useless or inefficient sorting algorithms. You may use them in your production application, altough I would strongly advise against that.
Currently, the following sorting algorithms are implemented:

- [Slowsort](https://github.com/Feliix42/sorting-rs/blob/master/src/slowsort.rs)
- [Bogosort](https://github.com/Feliix42/sorting-rs/blob/master/src/bogosort.rs)
- [Panicsort](https://github.com/Feliix42/sorting-rs/blob/master/src/panicsort.rs)

If you feel like this list misses some fancy algorithm (which it surely does!), you can implement it and [open a PR](https://github.com/Feliix42/sorting-rs/compare) or open an [issue](https://github.com/Feliix42/sorting-rs/issues/new). :)

## Implementation & Usage

All algorithms are currently implemented as traits for any `Vec<T>` where `T` fulfills the trait [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
So in order to use these sorting alrogithms, you can just invoke them on the object of interest:

```rust
extern crate sorting;

use sorting::*;

let unsorted = vec![5, 7, 8, 2, 1, 0];
unsorted.panicsort();   // will panic
```

Using the respective traits you can always implement these methods for your own structures.


