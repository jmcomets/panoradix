panoradix
=========

![Travis badge](https://travis-ci.org/jmcomets/panoradix.svg?branch=master)
![crates.io badge](https://img.shields.io/crates/v/panoradix.svg)

My take on implementing a [Radix tree][], for usage when large data mappings with
strings as indices.

Note: this is currently a **work in progress**, expect unannounced brutal API
changes every time the version is bumped.

## What's in this repo?

- [RadixMap][], a key-value map where the key is necessarily a string.
- [RadixSet][], a set of strings.

Both are backed by a [Radix tree][].

## What's being worked on?

A lot is missing right now, here's a wishlist sorted by difficulty/want:

- [ ] faster edge search: currently linear, should probably be binary search
- [ ] map iterators: `keys()`, `values()` and `values_mut()` for `RadixMap`
- [ ] erasing a key: should be straightforward
- [ ] intersection: should be straightforward
- [ ] union: since there can't be multiple values, merging two values with the
             same key should be annoying
- [ ] take arbitrary keys instead of strings: had a go trying to implement this
      early on, but the API transparency between `&str` and `String` is so damn
      gorgeous that I've postponed this indefinitely

[RadixMap]: https://github.com/jmcomets/panoradix/blob/master/src/map.rs
[RadixSet]: https://github.com/jmcomets/panoradix/blob/master/src/set.rs

[Radix tree]: https://en.wikipedia.org/wiki/Radix_tree
