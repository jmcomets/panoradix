use std::iter::FromIterator;

use tree::{
    Tree,
    Iter as TreeIter,
    Matches as TreeMatches,
};

use key::Key;

/// A map based on a [Radix tree](https://en.wikipedia.org/wiki/Radix_tree).
///
/// Radix trees are a implementation of the [Trie](https://en.wikipedia.org/wiki/Trie) data
/// structure, where any node can have N edges, and the keys are split on the edges to save memory.
/// There are no duplicate keys and values aren't only stored on the leaves of the tree.
///
/// This structure has the advantage of being fairly memory-efficient by compromising on key
/// insertion speed. There is also quite a bit of fragmentation due to the abundance of heap memory
/// usage in both the tree's structure and the data it contains.
///
/// You should probably only use this if you need to search by prefix in a large dataset of
/// strings. Consider using a sorted tree structure, such as a
/// [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) or any
/// implementation of a [Binary tree](https://en.wikipedia.org/wiki/Binary_tree) available on
/// [crates.io](https://crates.io/search?q=binary%20tree).
///
/// The `Key` trait is left private for safety (see the implementation for `str` for an
/// explanation). You can think of it as an abstraction over both `T` slices and `str` slices.
/// Therefore when specifying the type of `K`, you'll give either `[T]` or `str`.
pub struct RadixMap<K: Key + ?Sized, V> {
    tree: Tree<<K as Key>::Component, V>,
}

impl<K: Key + ?Sized, V> RadixMap<K, V> {
    /// Makes a new empty RadixMap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut map = RadixMap::new();
    ///
    /// // entries can now be inserted into the empty map
    /// map.insert("a", 1);
    /// ```
    pub fn new() -> RadixMap<K, V> {
        RadixMap { tree: Tree::new() }
    }

    /// Clears the map, removing all values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut m = RadixMap::new();
    /// m.insert("a", 1);
    /// m.clear();
    /// assert!(m.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.tree.clear();
    }

    /// Return the number of elements in the map.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut m = RadixMap::new();
    /// m.insert("a", 1);
    /// m.insert("b", 2);
    /// m.insert("c", 3);
    /// assert_eq!(m.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut map = RadixMap::new();
    /// assert_eq!(map.insert("a", 37), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert("a", 42);
    /// assert_eq!(map.insert("a", 1337), Some(42));
    /// ```
    pub fn insert(&mut self, key: &K, value: V) -> Option<V> {
        self.tree.insert(key.as_slice(), value)
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut map = RadixMap::new();
    /// map.insert("a", 1);
    /// assert_eq!(map.get("a"), Some(&1));
    /// assert_eq!(map.get("b"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        self.tree.get(key.as_slice())
    }

    /// Returns `true` if the map contains no elements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut a = RadixMap::new();
    /// assert!(a.is_empty());
    /// a.insert("a", ());
    /// assert!(!a.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut map = RadixMap::new();
    /// map.insert("a", 1);
    /// assert_eq!(map.remove("a"), Some(1));
    /// assert_eq!(map.remove("a"), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.tree.remove(key.as_slice())
    }

    /// Gets an iterator over the entries of the map, sorted by key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut map = RadixMap::new();
    /// map.insert("c", 3);
    /// map.insert("b", 2);
    /// map.insert("a", 1);
    ///
    /// for (key, value) in map.iter() {
    ///     println!("{}: {}", key, value);
    /// }
    ///
    /// let (first_key, first_value) = map.iter().next().unwrap();
    /// assert_eq!((first_key, *first_value), ("a".to_string(), 1));
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            iter: self.tree.iter(),
        }
    }

    /// Gets an iterator over the keys of the map (sorted).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut map = RadixMap::new();
    /// map.insert("c", 3);
    /// map.insert("b", 2);
    /// map.insert("a", 1);
    ///
    /// for key in map.keys() {
    ///     println!("{}", key);
    /// }
    ///
    /// let first_key = map.keys().next().unwrap();
    /// assert_eq!(first_key, "a".to_string());
    /// ```
    pub fn keys(&self) -> Keys<K, V> {
        Keys {
            iter: self.iter(),
        }
    }

    /// Gets an iterator over the values of the map, sorted by corresponding key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut map = RadixMap::new();
    /// map.insert("c", 3);
    /// map.insert("b", 2);
    /// map.insert("a", 1);
    ///
    /// for value in map.values() {
    ///     println!("{}", value);
    /// }
    ///
    /// let first_value = map.values().next().unwrap();
    /// assert_eq!(first_value, &1);
    /// ```
    pub fn values(&self) -> Values<K, V> {
        Values {
            iter: self.iter(),
        }
    }

    /// Gets an iterator over a filtered subset of the map, sorted by key.
    ///
    /// The iterator resembles `iter()` since it yields key-value pairs from the map. Note that
    /// the full key will be yielded each time, not just the filtered suffix.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixMap;
    ///
    /// let mut map = RadixMap::new();
    /// map.insert("abc", 1);
    /// map.insert("acd", 2);
    /// map.insert("abd", 3);
    /// map.insert("bbb", 1);
    /// map.insert("ccc", 1);
    ///
    /// for (key, value) in map.find("a") {
    ///     println!("{}: {}", key, value);
    /// }
    ///
    /// let (first_key, first_value) = map.find("a").next().unwrap();
    /// assert_eq!((first_key, first_value), ("abc".to_string(), &1));
    /// ```
    pub fn find<'a>(&'a self, key: &K) -> Matches<'a, K, V> {
        Matches {
            matches: self.tree.find(key.as_slice()),
        }
    }
}

impl<K: Key + ?Sized, V> Default for RadixMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, T> FromIterator<(T, V)> for RadixMap<K, V>
    where K: Key + ?Sized,
          T: AsRef<K>,
{
    fn from_iter<It>(iter: It) -> Self
        where It: IntoIterator<Item=(T, V)>,
    {
        let mut tree = Tree::new();
        for (t, v) in iter {
            tree.insert(t.as_ref().as_slice(), v);
        }

        RadixMap { tree: tree }
    }
}

/// An iterator over a `RadixMap`'s (key, value) pairs.
pub struct Iter<'a, K: 'a + Key + ?Sized, V: 'a> {
    iter: TreeIter<'a, K::Component, V>,
}

impl<'a, K: 'a + Key + ?Sized, V: 'a> Iterator for Iter<'a, K, V> {
    type Item = (K::Owned, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, v)| (K::from_vec(k), v))
    }
}

/// An iterator over a `RadixMap`'s keys.
pub struct Keys<'a, K: 'a + Key + ?Sized, V: 'a> {
    iter: Iter<'a, K, V>,
}

impl<'a, K: 'a + Key + ?Sized, V: 'a> Iterator for Keys<'a, K, V> {
    type Item = K::Owned;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

/// An iterator over a `RadixMap`'s values.
pub struct Values<'a, K: 'a + Key + ?Sized, V: 'a> {
    iter: Iter<'a, K, V>,
}

impl<'a, K: 'a + Key + ?Sized, V: 'a> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

/// An iterator over the elements matching a call to [`find`].
///
/// [`find`]: struct.RadixMap.html#method.find
pub struct Matches<'a, K: 'a + Key + ?Sized, V: 'a> {
    matches: TreeMatches<'a, K::Component, V>,
}

impl<'a, K: 'a + Key + ?Sized, V: 'a> Iterator for Matches<'a, K, V> {
    type Item = (K::Owned, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.matches.next().map(|(k, v)| (K::from_vec(k), v))
    }
}

#[cfg(test)]
mod tests {
    use super::RadixMap;

    #[test]
    fn it_can_lookup_elements() {
        let mut map: RadixMap<str, i32> = RadixMap::new();
        map.insert("a", 0);
        map.insert("ac", 1);

        let v = map.get("a");
        assert_eq!(v.map(|x| *x), Some(0));

        let v = map.get("ac");
        assert_eq!(v.map(|x| *x), Some(1));
    }

    #[test]
    fn it_has_a_key_iterator() {
        let mut map: RadixMap<str, ()> = RadixMap::new();
        map.insert("foo", ());
        map.insert("bar", ());
        map.insert("baz", ());

        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, vec!["bar", "baz", "foo"]);
    }

    #[test]
    fn it_has_a_value_iterator() {
        let mut map: RadixMap<str, i32> = RadixMap::new();
        map.insert("foo", 0);
        map.insert("bar", 1);
        map.insert("baz", 2);

        let values: Vec<_> = map.values().collect();
        assert_eq!(values, vec![&1, &2, &0]);
    }
}
