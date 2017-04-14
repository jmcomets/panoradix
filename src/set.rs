use map::RadixMap;

pub struct RadixSet {
    map: RadixMap<()>,
}

impl RadixSet {
    pub fn new() -> RadixSet {
        RadixSet { map: RadixMap::new() }
    }

    pub fn from_items<It, K>(items: It) -> RadixSet
        where It: IntoIterator<Item=K>,
              K: AsRef<str>,
    {
        let mut map = RadixMap::new();
        for k in items {
            map.insert(k.as_ref(), ());
        }

        RadixSet { map: map }
    }

    pub fn insert(&mut self, key: &str) {
        self.map.insert(key, ());
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.map.get(key).is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
