#![cfg(test)]

extern crate panoradix;

use panoradix::Tree;

#[test]
fn it_can_be_created() {
    let _: Tree<String, ()> = Tree::new();
}

#[test]
fn it_ignores_empty_elements() {
    let mut t: Tree<String, ()> = Tree::new();
    t.insert("", ());
    assert!(t.is_empty());
}

#[test]
fn it_accepts_an_element() {
    let mut t: Tree<String, ()> = Tree::new();
    t.insert("a", ());
    assert!(!t.is_empty());
}

#[test]
fn it_accepts_multiple_elements() {
    let mut t: Tree<String, ()> = Tree::new();
    t.insert("a", ());
    t.insert("b", ());
    t.insert("c", ());
    t.insert("ac", ());
    t.insert("ab", ());
    assert!(!t.is_empty());
}

#[test]
fn it_can_lookup_elements() {
    let mut t: Tree<String, i32> = Tree::new();
    t.insert("a", 0);
    t.insert("ac", 1);

    let v = t.get(&"a".to_string());
    assert_eq!(v.map(|x| *x), Some(0));

    let v = t.get(&"ac".to_string());
    assert_eq!(v.map(|x| *x), Some(1));
}

#[test]
fn it_can_be_built_from_multiple_elements() {
    let items: Vec<(String, ())> = vec![
        ("c".to_string(), ()),
        ("d".to_string(), ()),
        ("acb".to_string(), ()),
        ("b".to_string(), ()),
        ("a".to_string(), ()),
        ("ac".to_string(), ()),
    ];

    let t: Tree<String, ()> = Tree::from_items(items.clone());

    for (k, v) in items {
        let cloned_key = k.clone();
        let found = t.get(&k).map(|x| *x);
        assert_eq!((cloned_key, found), (k, Some(v)));
    }
}

#[test]
fn it_can_iterate_on_elements() {
    // TODO
}
