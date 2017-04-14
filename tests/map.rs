#![cfg(test)]

extern crate panoradix;

use panoradix::RadixMap;

#[test]
fn it_can_be_created() {
    let _: RadixMap<()> = RadixMap::new();
}

#[test]
fn it_ignores_empty_elements() {
    let mut map: RadixMap<()> = RadixMap::new();
    map.insert(&"".to_string(), ());
    //map.insert("".to_string(), ());
    //map.insert("", ());
    assert!(map.is_empty());
}

#[test]
fn it_accepts_an_element() {
    let mut map: RadixMap<()> = RadixMap::new();
    map.insert(&"a".to_string(), ());
    assert!(!map.is_empty());
}

#[test]
fn it_accepts_multiple_elements() {
    let mut map: RadixMap<()> = RadixMap::new();
    map.insert(&"a".to_string(), ());
    map.insert(&"b".to_string(), ());
    map.insert(&"c".to_string(), ());
    map.insert(&"ac".to_string(), ());
    map.insert(&"ab".to_string(), ());
    assert!(!map.is_empty());
}

#[test]
fn it_can_lookup_elements() {
    let mut map: RadixMap<i32> = RadixMap::new();
    map.insert(&"a".to_string(), 0);
    map.insert(&"ac".to_string(), 1);

    let v = map.get(&"a".to_string());
    assert_eq!(v.map(|x| *x), Some(0));

    let v = map.get(&"ac".to_string());
    assert_eq!(v.map(|x| *x), Some(1));
}

#[test]
fn it_can_be_built_from_multiple_elements() {
    let items = vec![
        ("a".to_string(),   ()),
        ("ac".to_string(),  ()),
        ("acb".to_string(), ()),
        ("b".to_string(),   ()),
        ("c".to_string(),   ()),
        ("d".to_string(),   ()),
    ];

    let map: RadixMap<()> = RadixMap::from_items(items.clone());

    for (k, v) in items {
        let cloned_key = k.clone();
        let found = map.get(&k).map(|x| *x);
        assert_eq!((cloned_key, found), (k, Some(v)));
    }
}

#[test]
fn it_can_iterate_on_elements() {
    // TODO
}
