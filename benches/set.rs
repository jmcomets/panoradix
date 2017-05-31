#![feature(test)]

extern crate test;
extern crate panoradix;

use std::iter;

use test::Bencher;
use panoradix::RadixSet;

#[bench]
fn basic_lookup(b: &mut Bencher) {
    let items = vec!["a", "abc", "bc", "abcd", "aebc"];
    let s: RadixSet<str> = items.iter().collect();
    b.iter(|| {
        for item in items.iter() {
            s.contains(item);
        }
    });
}

#[bench]
fn lookup_with_close_items(b: &mut Bencher) {
    let items: Vec<_> = (b'a'..b'z').map(|c| format!("{}-needle", c as char)).collect();

    let s: RadixSet<str> = items.iter().collect();
    b.iter(|| {
        s.contains("a-needle");
        s.contains("j-needle");
        s.contains("z-needle");
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
        items.iter().collect::<RadixSet<str>>();
    });
}

#[bench]
fn insert_repeating_characters_reversed(b: &mut Bencher) {
    let mut items = repeating_characters();
    items.reverse();

    b.iter(|| {
        items.iter().collect::<RadixSet<str>>();
    });
}

#[bench]
fn iteration(b: &mut Bencher) {
    {
        let s1: RadixSet<str> = WORDS_1.iter().collect();
        b.iter(|| { for _ in s1.iter() { /* noop */ } })
    }

    {
        let s2: RadixSet<str> = WORDS_2.iter().collect();
        b.iter(|| { for _ in s2.iter() { /* noop */ } })
    }

    {
        let s3: RadixSet<str> = WORDS_3.iter().collect();
        b.iter(|| { for _ in s3.iter() { /* noop */ } })
    }

    {
        let s4: RadixSet<str> = WORDS_4.iter().collect();
        b.iter(|| { for _ in s4.iter() { /* noop */ } })
    }
}

const WORDS_1: [&'static str; 50] = [
    "diaphragms",
    "lingered",
    "dec",
    "comet's",
    "advertiser",
    "patterned",
    "cubswin",
    "autocollimate",
    "equips",
    "monastic",
    "grinds",
    "visualizing",
    "depresses",
    "potentiating",
    "epitaphs",
    "apposition",
    "gesturing",
    "recognition's",
    "adjutant",
    "jostling",
    "burlesque",
    "regina",
    "al9agd",
    "urethra",
    "entrust",
    "dazed",
    "graywacke",
    "begetting",
    "polis",
    "neatest",
    "hose's",
    "unionize",
    "alcott",
    "popping",
    "motivating",
    "wetpussy",
    "stables",
    "medals",
    "mortally",
    "consecutive",
    "mischievousness",
    "verde",
    "permutations",
    "sibling",
    "separable",
    "ranchers",
    "isolation",
    "midwives",
    "interruptive",
    "antitrust",
    ];

const WORDS_2: [&'static str; 50] = [
    "microprogram",
    "star's",
    "clarification",
    "irreducible",
    "spontaneous",
    "deflect",
    "stevens",
    "whispering",
    "repaid",
    "dispense",
    "waterings",
    "sole",
    "penny",
    "prouder",
    "riptide",
    "peruvian",
    "koinonia",
    "unrepresentable",
    "vicinity",
    "cup",
    "felled",
    "exasperates",
    "overpowering",
    "obey",
    "sandstone",
    "packagers",
    "huzzah",
    "yak",
    "artemisia",
    "hurler",
    "pistachio",
    "transformer",
    "homesteaders",
    "mutilate",
    "coneflower",
    "nonetheless",
    "nina",
    "deathrate's",
    "driven",
    "gauntlet",
    "integer",
    "agamemnon",
    "delude",
    "tomatoes",
    "smocks",
    "carlo",
    "modal",
    "fullerton",
    "occurrence's",
    "stricken",
    ];

const WORDS_3: [&'static str; 50] = [
    "blasphemously",
    "served",
    "neuromuscular",
    "strews",
    "sectarian",
    "redden",
    "m's",
    "blackouts",
    "headaches",
    "shooting",
    "inexhaustible",
    "shelve",
    "gallstone",
    "appealing",
    "animatedly",
    "widower",
    "aversion",
    "impulse",
    "paternoster",
    "remarkableness",
    "visitor",
    "hastings",
    "sheds",
    "foolishly",
    "acumen",
    "treks",
    "inconsiderable",
    "claiming",
    "nonspecialist's",
    "revet",
    "andromed",
    "gibbous",
    "hymns",
    "jerkiness",
    "bushwhacking",
    "volunteering",
    "amphibole",
    "recognition",
    "gymnosperm",
    "wrens",
    "scabrous",
    "rags",
    "trainman",
    "excavate",
    "extension's",
    "subinterval's",
    "aphonic",
    "client's",
    "hornets",
    "rocks",
    ];

const WORDS_4: [&'static str; 50] = [
"tass",
"cowpox",
"distinctions",
"ear",
"tighter",
"espousing",
"pealed",
"seinfeld",
"fargo",
"nt5d27",
"insurrect",
"hotrod",
"oracular",
"affiliations",
"impracticality",
"bike",
"ballistic",
"spatterdock",
"her",
"surreal",
"chiefs",
"withholders",
"mosquito",
"professorial",
"cogent",
"instinctual",
"differentiated",
"quivering",
"bury",
"checkers",
"quarts",
"financiers",
"into",
"inscribing",
"stubbing",
"loudly",
"obstacles",
"ridge",
"robbing",
"logic's",
"smokescreen",
"preserved",
"vagary",
"thursdays",
"planeload",
"sentences",
"cajoled",
"multitude",
"quarry's",
"cemetery's",
    ];
