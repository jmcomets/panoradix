use tree::Tree;

pub struct RadixMap<V> {
    tree: Tree<V>,
}

impl<V> RadixMap<V> {
    pub fn new() -> RadixMap<V> {
        RadixMap { tree: Tree::new() }
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
}
