#![feature(test)]

extern crate test;
extern crate panoradix;

use std::iter;

use test::Bencher;
use panoradix::RadixSet;

#[bench]
fn basic_lookup(b: &mut Bencher) {
    let items = vec!["a", "abc", "bc", "abcd", "aebc"];
    let s: RadixSet = items.iter().collect();
    b.iter(|| {
        for item in items.iter() {
            s.has_key(item);
        }
    });
}

#[bench]
fn lookup_with_close_items(b: &mut Bencher) {
    let items: Vec<_> = (b'a'..b'z').map(|c| format!("{}-needle", c as char)).collect();

    let s: RadixSet = items.iter().collect();
    b.iter(|| {
        s.has_key("a-needle");
        s.has_key("j-needle");
        s.has_key("z-needle");
    });
}

fn repeating_characters() -> Vec<String> {
    const C: char = 'a';
    const N: usize = 20;

    let complete: String = iter::repeat(C).take(N).collect();
    (0..N-1).map(|n| complete[..n+1].to_string()).collect()
}

#[bench]
fn insert_repeating_characters(b: &mut Bencher) {
    let items = repeating_characters();

    b.iter(|| {
        items.iter().collect::<RadixSet>();
    });
}

#[bench]
fn insert_repeating_characters_reversed(b: &mut Bencher) {
    let mut items = repeating_characters();
    items.reverse();

    b.iter(|| {
        items.iter().collect::<RadixSet>();
    });
}
