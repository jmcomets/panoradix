panoradix
=========

My take on implementing a [Radix tree][], for usage when large data mappings
with slices as indices.

[![Travis badge](https://travis-ci.org/jmcomets/panoradix.svg?branch=master)](https://travis-ci.org/jmcomets/panoradix)
[![crates.io badge](https://img.shields.io/crates/v/panoradix.svg)](https://crates.io/crates/panoradix)

[Documentation][]

## What's in this repo?

- [RadixMap][], a key-value map.
- [RadixSet][], a set of keys.

Both are backed by a [Radix tree][].

Any slice of elements that are `Ord + Eq + Clone` can be used as keys, as well
as `str` that are taken as byte slices. Any lookups are done using a `&[T]` and
iteration will yield an owned `Vec<T>` each time (for `str` it will yield
`String` items).

Further extension of keys is possible but not recommended since the keys are
arguably always a `[T]`. If you really want to do this, have a look at the
`ExtensibleKey` trait.

## Examples

### Insert / Lookup

```rust
let mut map: RadixMap<str, i32> = RadixMap::new();
map.insert("a", 0);
map.insert("ac", 1);

assert_eq!(map.get("a"), Some(&0));
assert_eq!(map.get("ac"), Some(&1));
assert_eq!(map.get("ab"), None);
```

### Removal

```rust
let v = vec!["foo", "bar", "baz"];
let mut set: RadixSet<str> = RadixSet::from_iter(v);

set.remove("bar");
assert!(!set.contains("bar"));
assert!(set.contains("baz"));
```

### Completion

```rust
let v = vec!["foo", "bar", "baz"];
let set: RadixSet<str> = RadixSet::from_iter(v);

assert_eq!(set.find("ba").collect::<Vec<_>>(), vec!["bar", "baz"]);
```

[Radix tree]: https://en.wikipedia.org/wiki/Radix_tree

[Documentation]: https://docs.rs/panoradix

[RadixMap]: https://github.com/jmcomets/panoradix/blob/master/src/map.rs
[RadixSet]: https://github.com/jmcomets/panoradix/blob/master/src/set.rs

## Contributing

I try to maintain a list of things that need to be worked on [over
here](https://github.com/jmcomets/panoradix/blob/master/TODO.md). Issues / PRs
are always welcome!
