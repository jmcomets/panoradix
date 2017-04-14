pub type Tree<V> = Node<V>;

pub struct Node<V> {
    prefix: String,
    value: Option<V>,
    children: Vec<Box<Node<V>>>,
}

impl<V> Node<V> {
    pub fn new() -> Node<V> {
        Node {
            prefix: String::default(),
            value: None,
            children: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.prefix.is_empty() && self.children.iter().all(|n| n.is_empty())
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        match self.search(key) {
            NodeSearch::InChild(suffix, i) => self.children[i].get(&suffix),
            NodeSearch::Found(i)           => self.children[i].value.as_ref(),
            _                              => None,
        }
    }

    pub fn insert(&mut self, key: &str, value: V) {
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

    fn boxed(prefix: String, value: V) -> Box<Node<V>> {
        Box::new(Node {
            prefix: prefix,
            value: Some(value),
            children: Vec::new(),
        })
    }

    fn search(&self, key: &str) -> NodeSearch {
        let suffix = get_suffix_between(&self.prefix, key);
        if suffix.is_empty() {
            return NodeSearch::NotFound
        }

        let search = self.children.binary_search_by_key(&(&suffix), |n| &n.prefix);
        match search {
            Ok(i) => NodeSearch::Found(i),
            Err(i) => {
                let mut child_index = None;

                // left child
                if i > 0 {
                    let ref child = self.children[i - 1];
                    if has_common_prefix(&child.prefix, &suffix) {
                        child_index = Some(i - 1);
                    }
                }

                // right child
                if i + 1 < self.children.len() {
                    let ref child = self.children[i + 1];
                    if has_common_prefix(&child.prefix, &suffix) {
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

enum NodeSearch {
    Found(usize),
    InChild(String, usize),
    CanAddToChildren(String, usize),
    NotFound,
}

fn has_common_prefix(first: &str, second: &str) -> bool {
    first.chars().zip(second.chars())
        .take_while(|&(ref a, ref b)| a == b)
        .next().is_some()
}

fn get_suffix_between(first: &str, second: &str) -> String {
    let nb = first.chars()
        .zip(second.chars())
        .take_while(|&(a, b)| a == b)
        .count();
    if nb == 0 {
        second.to_string()
    } else if nb < first.len() {
        let (_, suffix) = first.split_at(nb);
        suffix.to_string()
    } else if nb < second.len() {
        let (_, suffix) = second.split_at(nb);
        suffix.to_string()
    } else {
        "".to_string()
    }
}
