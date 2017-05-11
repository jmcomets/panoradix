use tree::Tree;

// re-exports from the private `tree` module
pub use tree::{
    Matches,
    Iter,
};

pub struct RadixMap<V> {
    tree: Tree<V>,
}

impl<V> RadixMap<V> {
    pub fn new() -> RadixMap<V> {
        RadixMap { tree: Tree::new() }
    }

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

    pub fn insert(&mut self, key: &str, value: V) -> Option<V> {
        self.tree.insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.tree.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.tree.remove(key)
    }

    pub fn iter(&self) -> Iter<V> {
        self.tree.iter()
    }

    pub fn keys(&self) -> Keys<V> {
        Keys {
            iter: self.tree.iter(),
        }
    }

    pub fn values(&self) -> Values<V> {
        Values {
            iter: self.tree.iter(),
        }
    }

    pub fn find<'a>(&'a self, key: &str) -> Matches<'a, V> {
        self.tree.find(key)
    }
}

impl<V> Default for RadixMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Keys<'a, V: 'a> {
    iter: Iter<'a, V>,
}

impl<'a, V: 'a> Iterator for Keys<'a, V> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

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
