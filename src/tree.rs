pub type Tree<K, V> = Node<K, V>;

pub struct Node<K, V> {
    key: K,
    value: Option<V>,
    children: Vec<Box<Node<K, V>>>,
}

impl<K: Key, V> Node<K, V> {
    pub fn new() -> Node<K, V> {
        Node {
            key: K::default(),
            value: None,
            children: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.key.is_empty() && self.children.iter().all(|n| n.is_empty())
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.search(key) {
            NodeSearch::InChild(suffix, i) => self.children[i].get(&suffix),
            NodeSearch::Found(i)           => self.children[i].value.as_ref(),
            _                              => None,
        }
    }

    pub fn insert(&mut self, key: &K, value: V) {
        match self.search(key) {
            NodeSearch::InChild(suffix, i)   => {
                let ref mut child = self.children[i];
                child.insert(&suffix, value);
            },
            NodeSearch::CanAddToChildren(suffix, i) => {
                let node = Node::boxed(suffix, value);
                self.children.insert(i, node);
            },
            NodeSearch::Found(_) | NodeSearch::NotFound => {}
        }
    }

    fn boxed(key: K, value: V) -> Box<Node<K, V>> {
        Box::new(Node {
            key: key,
            value: Some(value),
            children: Vec::new(),
        })
    }

    fn search(&self, key: &K) -> NodeSearch<K> {
        let suffix = suffix_with(&self.key, key);
        if suffix.is_empty() {
            return NodeSearch::NotFound
        }

        // Note: this needs to be a &&K because we don't want to move the key (K) out of
        // context.
        let search = self.children.binary_search_by_key(&(&suffix), |n| &n.key);
        match search {
            Ok(i) => NodeSearch::Found(i),
            Err(i) => {
                let mut child_index = None;

                // left child
                if i > 0 {
                    let ref child = self.children[i - 1];
                    if has_common_prefix(&child.key, &suffix) {
                        child_index = Some(i - 1);
                    }
                }

                // right child
                if i + 1 < self.children.len() {
                    let ref child = self.children[i + 1];
                    if has_common_prefix(&child.key, &suffix) {
                        child_index = Some(i + 1);
                    }
                }

                match child_index {
                    Some(i) => NodeSearch::InChild(suffix, i),
                    None    => NodeSearch::CanAddToChildren(suffix, i),
                }
            }
        }
    }
}

enum NodeSearch<K: Key> {
    Found(usize),
    InChild(K, usize),
    CanAddToChildren(K, usize),
    NotFound,
}

pub trait Key: Default + Eq + Ord {
    fn is_empty(&self) -> bool;

    fn get_prefix_and_suffix(&self, other: &Self) -> (Self, Self);
}

impl Key for String {
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

fn has_common_prefix<K: Key>(key: &K, other: &K) -> bool {
    let (prefix, _) = key.get_prefix_and_suffix(other);
    !prefix.is_empty()
}

fn suffix_with<K: Key>(key: &K, other: &K) -> K {
    let (_, suffix) = key.get_prefix_and_suffix(other);
    suffix
}

#[allow(dead_code)]
fn has_common_suffix<K: Key>(key: &K, other: &K) -> bool {
    let (_, suffix) = key.get_prefix_and_suffix(other);
    !suffix.is_empty()
}

#[allow(dead_code)]
fn prefix_with<K: Key>(key: &K, other: &K) -> K {
    let (prefix, _) = key.get_prefix_and_suffix(other);
    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_the_right_common_prefix() {
        let a = "a".to_string();
        let abc = "abc".to_string();
        let ac = "ac".to_string();
        let b = "b".to_string();
        let bc = "bc".to_string();
        let c = "c".to_string();
        assert_eq!(suffix_with(&a, &b), b);
        assert_eq!(suffix_with(&a, &ac), c);
        assert_eq!(suffix_with(&a, &abc), bc);
    }

    #[test]
    fn it_handles_empty_strings() {
        let a = "a".to_string();
        let empty = "".to_string();
        assert_eq!(suffix_with(&a, &empty), empty);
        assert_eq!(suffix_with(&empty, &a), a);
        assert_eq!(suffix_with(&empty, &empty), empty);
    }

    #[test]
    fn it_says_if_it_has_a_common_suffix() {
        let a = "a".to_string();
        let d = "d".to_string();
        assert!(!has_common_prefix(&a, &d));
        assert!(!has_common_prefix(&d, &a));
    }
}
