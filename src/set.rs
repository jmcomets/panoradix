use map::RadixMap;
use map::Keys as MapKeys;

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

    pub fn iter<'a>(&'a self) -> Keys<'a> {
        self.keys()
    }

    pub fn keys<'a>(&'a self) -> Keys<'a> {
        Keys {
            iter: self.map.keys(),
        }
    }
}

pub struct Keys<'a> {
    iter: MapKeys<'a, ()>,
}

impl<'a> Iterator for Keys<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::RadixSet;
    use utils::IntoSortedVec;

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
    fn it_has_a_key_iterator() {
        let mut map = RadixSet::new();
        map.insert("foo");
        map.insert("bar");
        map.insert("baz");

        assert_eq!(vec!["bar", "baz", "foo"], map.keys().into_sorted_vec());
    }
}
