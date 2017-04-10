pub trait Path: Default + Eq + Ord + Clone {
    fn is_empty(&self) -> bool;

    fn get_prefix_and_suffix(&self, other: &Self) -> (Self, Self);

    fn has_common_prefix(&self, other: &Self) -> bool {
        let (prefix, _) = self.get_prefix_and_suffix(other);
        !prefix.is_empty()
    }

    fn prefix_with(&self, other: &Self) -> Self {
        let (prefix, _) = self.get_prefix_and_suffix(other);
        prefix
    }

    fn has_common_suffix(&self, other: &Self) -> bool {
        let (_, suffix) = self.get_prefix_and_suffix(other);
        !suffix.is_empty()
    }

    fn suffix_with(&self, other: &Self) -> Self {
        let (_, suffix) = self.get_prefix_and_suffix(other);
        suffix
    }
}

impl Path for String {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get_prefix_and_suffix(&self, other: &String) -> (String, String) {
        let nb = self.chars()
            .zip(other.chars())
            .take_while(|&(a, b)| a == b)
            .count();
        if nb == 0 {
            ("".to_string(), other.clone())
        } else if nb < self.len() {
            let (prefix, suffix) = self.split_at(nb);
            (prefix.to_string(), suffix.to_string())
        } else if nb < other.len() {
            let (prefix, suffix) = other.split_at(nb);
            (prefix.to_string(), suffix.to_string())
        } else {
            (self.clone(), "".to_string())
        }
    }
}
