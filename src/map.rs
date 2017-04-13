use tree::{Tree, Key};

pub struct RadixMap<K: Key, V> {
    tree: Tree<K, V>,
}

impl<K: Key, V> RadixMap<K, V> {
    pub fn new() -> RadixMap<K, V> {
        RadixMap { tree: Tree::new() }
    }

    pub fn from_items<It>(items: It) -> RadixMap<K, V>
        where It: IntoIterator<Item=(K, V)>,
    {
        let mut tree = Tree::new();
        for (k, v) in items {
            tree.insert(&k, v);
        }

        RadixMap { tree: tree }
    }

    pub fn insert(&mut self, key: &K, value: V) {
        self.tree.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.tree.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
}
