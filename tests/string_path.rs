#![cfg(test)]

extern crate panoradix;

use panoradix::Path;

#[test]
fn it_gets_the_right_common_prefix() {
    let a = "a".to_string();
    let abc = "abc".to_string();
    let ac = "ac".to_string();
    let b = "b".to_string();
    let bc = "bc".to_string();
    let c = "c".to_string();
    assert_eq!(a.suffix_with(&b), b);
    assert_eq!(a.suffix_with(&ac), c);
    assert_eq!(a.suffix_with(&abc), bc);
}

#[test]
fn it_handles_empty_strings() {
    let a = "a".to_string();
    let empty = "".to_string();
    assert_eq!(a.suffix_with(&empty), empty);
    assert_eq!(empty.suffix_with(&a), a);
    assert_eq!(empty.suffix_with(&empty), empty);
}

#[test]
fn it_says_if_it_has_a_common_suffix() {
    let a = "a".to_string();
    let d = "d".to_string();
    assert!(!a.has_common_prefix(&d));
    assert!(!d.has_common_prefix(&a));
}
