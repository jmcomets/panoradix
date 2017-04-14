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

    pub fn insert(&mut self, key: &str, value: V) {
        self.tree.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.tree.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
}
