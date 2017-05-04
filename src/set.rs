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

    pub fn insert(&mut self, key: &str) -> bool {
        self.map.insert(key, ()).is_none()
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.map.get(key).is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
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

        let set: RadixSet = RadixSet::from_items(items.clone());

        assert!(items.iter().all(|k| set.has_key(k)))
    }

    #[test]
    fn it_can_iterate_on_elements() {
        // TODO
    }
}
