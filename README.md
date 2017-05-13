panoradix
=========

My take on implementing a [Radix tree][], for usage when large data mappings with
strings as indices.

[![Travis badge](https://travis-ci.org/jmcomets/panoradix.svg?branch=master)](https://travis-ci.org/jmcomets/panoradix)
[![crates.io badge](https://img.shields.io/crates/v/panoradix.svg)](https://crates.io/crates/panoradix)

[Documentation][]

Note: this is currently a **work in progress**, expect unannounced brutal API
changes every time the version is bumped.

## What's in this repo?

- [RadixMap][], a key-value map where the key is necessarily a string.
- [RadixSet][], a set of strings.

Both are backed by a [Radix tree][].

## What's being worked on?

A lot is missing right now, here's a wishlist sorted by difficulty/want:

- [ ] documentation: should follow the [API guidelines](https://github.com/brson/rust-api-guidelines)
  - in progress, some prose needs to be written on the modules

- [ ] map iterators: `values_mut()`
- [ ] intersection: should be straightforward
- [ ] union: since there can't be multiple values, merging two values with the
             same key should be annoying
- [ ] take arbitrary keys instead of strings: had a go trying to implement this
      early on, but the API transparency between `&str` and `String` is so damn
      gorgeous that I've postponed this indefinitely
- [x] faster edge search: currently linear, should probably be binary search
  - can be found on the [binary-search-edges][] branch, needs to be
    optimized as benches show slower runs using binary search instead of linear

### What's just been finished?

- [x] faster iteration: tree iterators were hacked together and abuses
                        heap allocation/recursion, now use a faster structure
- [x] clearing: `clear()` on both map/set
- [x] fitering elements: `find()` on both map/set
- [x] erasing a key: `remove()` on both map/set
- [x] set iterators: `keys()`
- [x] map iterators: `keys()`, `values()` and `iter()`

[Radix tree]: https://en.wikipedia.org/wiki/Radix_tree

[Documentation]: https://docs.rs/panoradix

[RadixMap]: https://github.com/jmcomets/panoradix/blob/master/src/map.rs
[RadixSet]: https://github.com/jmcomets/panoradix/blob/master/src/set.rs

[binary-search-edges]: https://github.com/jmcomets/panoradix/tree/binary-search-edges
