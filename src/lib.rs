//! panoradix is a set of structures based of the [Radix
//! tree](https://en.wikipedia.org/wiki/Radix_tree) data structure, optimized for indexing strings
//! "by prefix".

#![deny(missing_docs)]

// Clippy lints
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

pub use map::RadixMap;
pub use set::RadixSet;

mod map;
mod key;
mod set;

mod tree;
