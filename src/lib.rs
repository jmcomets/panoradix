//! panoradix is a set of structures based of the [Radix
//! tree](https://en.wikipedia.org/wiki/Radix_tree) data structure, optimized for indexing strings
//! "by prefix".

#![deny(missing_docs)]

pub use map::RadixMap;
pub use set::RadixSet;
pub use key::ExtensibleKey as RadixKey;

/// Module containing a map based on a [Radix tree](https://en.wikipedia.org/wiki/Radix_tree).
pub mod map;

/// Module containing a set based on a [Radix tree](https://en.wikipedia.org/wiki/Radix_tree).
pub mod set;

mod key;
mod tree;
