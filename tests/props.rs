#[macro_use]
extern crate proptest;
extern crate panoradix;

extern crate rand;

use proptest::prelude::*;
use panoradix::RadixSet;

use rand::{Rng, StdRng, SeedableRng};

proptest! {
    #[test]
    fn test_insertion_coherence(ref items in any::<Vec<String>>()) {
        let tree: RadixSet<str> = items.iter().collect();
        assert!(items.iter().all(|x| tree.contains(x)));
    }

    #[test]
    fn test_insertion_coherence_shuffled(
        ref items in any::<Vec<String>>(),
        nb_shuffles in 1..10usize,
        seed in any::<[u8; 32]>()
        ) {
        let mut rng = StdRng::from_seed(seed);

        for _ in 0..nb_shuffles {
            let mut cloned_items: Vec<_> = items.iter().cloned().collect();
            rng.shuffle(&mut cloned_items);

            let tree: RadixSet<str> = items.iter().collect();
            assert!(items.iter().all(|x| tree.contains(x)));
        }
    }
}
