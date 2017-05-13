use std::iter::FromIterator;

use map::{
    RadixMap,
    Matches as MapMatches,
    Keys as MapKeys,
};

/// A set based on a [Radix tree](https://en.wikipedia.org/wiki/Radix_tree).
///
/// TODO: section on benefits/drawbacks of using a Radix tree
pub struct RadixSet {
    map: RadixMap<()>,
}

impl RadixSet {
    /// Makes a new empty RadixSet.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixSet;
    ///
    /// let mut set = RadixSet::new();
    ///
    /// // entries can now be inserted into the empty set
    /// set.insert("a");
    /// ```
    pub fn new() -> RadixSet {
        RadixSet { map: RadixMap::new() }
    }

    /// Clears the set, removing all values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixSet;
    ///
    /// let mut set = RadixSet::new();
    /// set.insert("a");
    /// set.clear();
    /// assert!(set.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Inserts a key into the set.
    ///
    /// If the set did not have this key present, `true` is returned, otherwise `false` is
    /// returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixSet;
    ///
    /// let mut set = RadixSet::new();
    /// assert_eq!(set.insert("a"), true);
    /// assert_eq!(set.is_empty(), false);
    ///
    /// assert_eq!(set.insert("a"), false);
    /// ```
    pub fn insert(&mut self, key: &str) -> bool {
        self.map.insert(key, ()).is_none()
    }

    /// Returns if the key is present in the set.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixSet;
    ///
    /// let mut set = RadixSet::new();
    /// set.insert("a");
    /// assert_eq!(set.has_key("a"), true);
    /// assert_eq!(set.has_key("b"), false);
    /// ```
    pub fn has_key(&self, key: &str) -> bool {
        self.map.get(key).is_some()
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixSet;
    ///
    /// let mut set = RadixSet::new();
    /// assert!(set.is_empty());
    /// set.insert("a");
    /// assert!(!set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Removes a key from the set, returning if the key was previously in the map.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixSet;
    ///
    /// let mut set = RadixSet::new();
    /// set.insert("a");
    /// assert_eq!(set.remove("a"), true);
    /// assert_eq!(set.remove("a"), false);
    /// ```
    pub fn remove(&mut self, key: &str) -> bool {
        self.map.remove(key).is_some()
    }

    /// Gets an iterator over the keys of the map (sorted).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixSet;
    ///
    /// let mut set = RadixSet::new();
    /// set.insert("c");
    /// set.insert("b");
    /// set.insert("a");
    ///
    /// for key in set.keys() {
    ///     println!("{}", key);
    /// }
    ///
    /// let first_key = set.keys().next().unwrap();
    /// assert_eq!(first_key, "a".to_string());
    /// ```
    pub fn keys(&self) -> Keys {
        self.map.keys()
    }

    /// Gets an iterator over the keys of the map (sorted).
    ///
    /// This method is strictly equivalent to the `keys()` method.
    pub fn iter(&self) -> Iter {
        self.keys()
    }

    /// Gets an iterator over a filtered subset of the set (sorted).
    ///
    /// Note that the full key will be yielded each time, not just the filtered suffix.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use panoradix::RadixSet;
    ///
    /// let mut set = RadixSet::new();
    /// set.insert("abc");
    /// set.insert("acd");
    /// set.insert("abd");
    /// set.insert("bbb");
    /// set.insert("ccc");
    ///
    /// for key in set.find("a") {
    ///     println!("{}", key);
    /// }
    ///
    /// let first_key = set.find("a").next().unwrap();
    /// assert_eq!(first_key, "abc".to_string());
    /// ```
    pub fn find<'a>(&'a self, key: &str) -> Matches<'a> {
        Matches {
            iter: self.map.find(key),
        }
    }
}

impl Default for RadixSet {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: AsRef<str>> FromIterator<K> for RadixSet {
    fn from_iter<It>(iter: It) -> Self
        where It: IntoIterator<Item=K>,
    {
        let iter = iter.into_iter().map(|k| (k, ()));
        RadixSet { map: RadixMap::from_iter(iter), }
    }
}

/// An iterator over a `RadixSet`'s keys.
pub type Keys<'a> = MapKeys<'a, ()>;

/// An alias for `Keys`.
pub type Iter<'a> = Keys<'a>;

pub struct Matches<'a> {
    iter: MapMatches<'a, ()>,
}

impl<'a> Iterator for Matches<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

#[cfg(test)]
mod tests {
    use super::RadixSet;

    #[test]
    fn it_can_be_created() {
        let _: RadixSet = RadixSet::new();
    }

    #[test]
    fn it_accepts_an_empty_element() {
        let mut set: RadixSet = RadixSet::new();
        set.insert("");
        assert!(!set.is_empty());
    }

    #[test]
    fn it_accepts_an_element() {
        let mut set: RadixSet = RadixSet::new();
        set.insert("a");
        assert!(!set.is_empty());
    }

    #[test]
    fn it_accepts_multiple_elements() {
        let mut set: RadixSet = RadixSet::new();
        set.insert("a");
        set.insert("b");
        set.insert("c");
        set.insert("ac");
        set.insert("ab");
        assert!(!set.is_empty());
    }

    #[test]
    fn it_can_be_built_from_multiple_elements() {
        let items = vec!["a", "ac", "acb", "b", "c", "d"];

        let set: RadixSet = items.iter().collect();

        assert!(items.iter().all(|k| set.has_key(k)))
    }

    #[test]
    fn it_has_a_key_iterator() {
        let mut map = RadixSet::new();
        map.insert("foo");
        map.insert("bar");
        map.insert("baz");

        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, vec!["bar", "baz", "foo"]);
    }
}
