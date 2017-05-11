use tree::Tree;

// re-exports from the private `tree` module
pub use tree::{
    Matches,
    Iter,
};

/// A map based on a [Radix tree](https://en.wikipedia.org/wiki/Radix_tree).
///
/// TODO: section on benefits/drawbacks of using a Radix tree
pub struct RadixMap<V> {
    tree: Tree<V>,
}

impl<V> RadixMap<V> {
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
    pub fn new() -> RadixMap<V> {
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

    pub fn from_items<It, K>(items: It) -> RadixMap<V>
        where It: IntoIterator<Item=(K, V)>,
              K: AsRef<str>,
    {
        let mut tree = Tree::new();
        for (k, v) in items {
            tree.insert(k.as_ref(), v);
        }

        RadixMap { tree: tree }
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
    pub fn insert(&mut self, key: &str, value: V) -> Option<V> {
        self.tree.insert(key, value)
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
    pub fn get(&self, key: &str) -> Option<&V> {
        self.tree.get(key)
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
    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.tree.remove(key)
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
    pub fn iter(&self) -> Iter<V> {
        self.tree.iter()
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
    pub fn keys(&self) -> Keys<V> {
        Keys {
            iter: self.tree.iter(),
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
    pub fn values(&self) -> Values<V> {
        Values {
            iter: self.tree.iter(),
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
    pub fn find<'a>(&'a self, key: &str) -> Matches<'a, V> {
        self.tree.find(key)
    }
}

impl<V> Default for RadixMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

/// An iterator over a `RadixMap`'s keys.
pub struct Keys<'a, V: 'a> {
    iter: Iter<'a, V>,
}

impl<'a, V: 'a> Iterator for Keys<'a, V> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

/// An iterator over a `RadixMap`'s values.
pub struct Values<'a, V: 'a> {
    iter: Iter<'a, V>,
}

impl<'a, V: 'a> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

#[cfg(test)]
mod tests {
    use super::RadixMap;

    #[test]
    fn it_can_lookup_elements() {
        let mut map: RadixMap<i32> = RadixMap::new();
        map.insert("a", 0);
        map.insert("ac", 1);

        let v = map.get("a");
        assert_eq!(v.map(|x| *x), Some(0));

        let v = map.get("ac");
        assert_eq!(v.map(|x| *x), Some(1));
    }

    #[test]
    fn it_has_a_key_iterator() {
        let mut map: RadixMap<()> = RadixMap::new();
        map.insert("foo", ());
        map.insert("bar", ());
        map.insert("baz", ());

        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, vec!["bar", "baz", "foo"]);
    }

    #[test]
    fn it_has_a_value_iterator() {
        let mut map: RadixMap<i32> = RadixMap::new();
        map.insert("foo", 0);
        map.insert("bar", 1);
        map.insert("baz", 2);

        let values: Vec<_> = map.values().collect();
        assert_eq!(values, vec![&1, &2, &0]);
    }
}
