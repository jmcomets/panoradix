use std::cmp::Ord;

pub trait IntoSortedVec {
    type Item: Ord;

    fn into_sorted_vec(self) -> Vec<Self::Item>;
}

impl<It> IntoSortedVec for It
    where It: Iterator,
            It::Item: Ord,
{
    type Item = <It as Iterator>::Item;

    fn into_sorted_vec(self) -> Vec<Self::Item> {
        let mut v: Vec<_> = self.collect();
        v.sort();
        v
    }
}

