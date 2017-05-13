use std::mem;
use std::slice;

pub type Tree<V> = Node<V>;

pub struct Node<V> {
    value: Option<V>,
    edges: Vec<Edge<V>>,
}

impl<V> Node<V> {
    pub fn new() -> Node<V> {
        Node {
            value: None,
            edges: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        *self = Node::new();
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.edges.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        if key.is_empty() {
            self.value.as_ref()
        } else if let Some((i, PrefixCmp::Full(suffix))) = self.search_for_prefix(key) {
            self.edges[i].node.get(suffix)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: &str, value: V) -> Option<V> {
        if key.is_empty() {
            let old_value = self.value.take();
            self.value = Some(value);
            old_value
        } else {
            if let Some((i, cmp)) = self.search_for_prefix(key) {
                match cmp {
                    // Full prefix: insert in the child
                    PrefixCmp::Full(suffix) => {
                        return self.edges[i].node.insert(suffix, value);
                    },

                    // Partial prefix: split the key and replace the edge's node with a new one
                    // that holds both nodes to insert.
                    PrefixCmp::Partial(j) => {
                        self.edges[i].split_insert(j, key, value);
                    },
                };
            } else {
                // No match in edges: insert a new edge
                let new_edge = Edge::new(key.to_string(), Some(value));

                // TODO: this should be revamped along with `search_for_prefix`
                let i = self.edges.binary_search_by(|e| e.prefix.as_str().cmp(key)).unwrap_err();
                self.edges.insert(i, new_edge);
            }

            None
        }
    }

    pub fn iter(&self) -> Iter<V> {
        Iter::new(self)
    }

    pub fn remove(&mut self, key: &str) -> Option<V> {
        if key.is_empty() {
            self.value.take()
        } else if let Some((i, cmp)) = self.search_for_prefix(key) {
            match cmp {
                PrefixCmp::Full(suffix) => {
                    let ret = self.edges[i].node.remove(suffix);

                    if self.edges[i].node.is_empty() {
                        self.edges.remove(i);
                    }

                    ret
                },
                PrefixCmp::Partial(_) => None,
            }
        } else {
            None
        }
    }

    pub fn find<'a>(&'a self, key: &str) -> Matches<'a, V> {
        self.find_subtree(key, "".to_string())
    }

    fn find_subtree<'a>(&'a self, key: &str, mut prefix: String) -> Matches<'a, V> {
        if key.is_empty() {
            Matches::found(prefix, self)
        } else if let Some((i, PrefixCmp::Full(suffix))) = self.search_for_prefix(key) {
            // concatenate the prefix used to get here with the current full prefix
            prefix += {
                if suffix.is_empty() {
                    key
                } else {
                    let (p, _) = key.split_at(key.len() - suffix.len());
                    p
                }
            };

            self.edges[i].node.find_subtree(suffix, prefix)
        } else {
            Matches::none()
        }
    }

    fn search_for_prefix<'a>(&self, key: &'a str) -> Option<(usize, PrefixCmp<'a>)> {
        self.edges.iter()
            .enumerate()
            .flat_map(|(i, e)| e.cmp_prefix(key).map(|cmp| (i, cmp)))
            .next()
    }
}

struct Edge<V> {
    prefix: String,
    node: Box<Node<V>>,
}

impl<V> Edge<V> {
    fn new(prefix: String, value: Option<V>) -> Edge<V> {
        let mut node = Box::new(Node::new());
        node.value = value;

        Edge {
            prefix: prefix,
            node: node,
        }
    }

    fn split_insert(&mut self, i: usize, key: &str, value: V) {
        let (prefix, (key_suffix, edge_suffix)) = {
            let (prefix, key_suffix) = key.split_at(i);
            let (_, edge_suffix) = self.prefix.split_at(i);

            (prefix.to_string(), (key_suffix.to_string(), edge_suffix.to_string()))
        };

        // assign the new prefix
        self.prefix = prefix;

        // move out the node's value for future use
        let moved_value = self.node.value.take();

        // swap the old and new node's edges
        let mut new_edges = Vec::with_capacity(2);
        mem::swap(&mut self.node.edges, &mut new_edges);

        let mut moved_edge = Edge::new(edge_suffix, moved_value);
        moved_edge.node.edges = new_edges;

        // update the parent edge: if the key is contained in the existing prefix, then it should
        // be exactly equal to the prefix of the parent edge, hence the parent edge's value should
        // be updated with the value we're trying to insert
        self.node.edges.push(moved_edge);
        if !key_suffix.is_empty() {
            self.node.edges.push(Edge::new(key_suffix, Some(value)));
        } else {
            self.node.value = Some(value);
        }
        // finally, make sure the edges are sorted by prefix
        self.node.edges.sort_by(|a, b| a.prefix.cmp(&b.prefix));
    }

    fn cmp_prefix<'a>(&self, key: &'a str) -> Option<PrefixCmp<'a>> {
        cmp_prefix(&self.prefix, key)
    }
}

enum PrefixCmp<'a> {
    Full(&'a str),
    Partial(usize),
}

fn cmp_prefix<'a>(haystack: &str, needle: &'a str) -> Option<PrefixCmp<'a>> {
    let nb = haystack.chars().zip(needle.chars())
        .take_while(|&(a, b)| a == b)
        .count();
    if nb == 0 {
        None
    } else if nb < haystack.len() {
        Some(PrefixCmp::Partial(nb))
    } else if nb < needle.len() {
        let (_, suffix) = needle.split_at(nb);
        Some(PrefixCmp::Full(suffix))
    } else {
        Some(PrefixCmp::Full(""))
    }
}

pub struct Iter<'a, V: 'a> {
    path: Vec<IterPath<'a, V>>,
    prefix: String,
}

impl<'a, V: 'a> Iter<'a, V> {
    fn new(node: &'a Node<V>) -> Iter<'a, V> {
        Iter {
            path: vec![IterPath::from_node(node)],
            prefix: String::new(),
        }
    }
}

impl<'a, V: 'a> Iterator for Iter<'a, V> {
    type Item = (String, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while !self.path.is_empty() {
            if let Some(adv) = self.path.last_mut().unwrap().advance() {
                match adv {
                    Ok(value) => {
                        return Some((self.prefix.clone(), value));
                    },
                    Err(elem) => {
                        self.prefix += elem.prefix;
                        self.path.push(elem);
                    },
                }
            } else {
                let last_prefix = self.path.pop().unwrap().prefix;
                let i = self.prefix.len()-last_prefix.len();
                self.prefix.drain(i..);
            }
        }

        None

    }
}

struct IterPath<'a, V: 'a> {
    node: &'a Node<V>,
    edge_iter: Option<slice::Iter<'a, Edge<V>>>,
    prefix: &'a str,
}

impl<'a, V: 'a> IterPath<'a, V> {
    fn from_node(node: &'a Node<V>) -> IterPath<'a, V> {
        IterPath {
            node: node,
            prefix: "",
            edge_iter: None,
        }
    }

    fn from_edge(edge: &'a Edge<V>) -> IterPath<'a, V> {
        IterPath {
            node: &edge.node,
            prefix: &edge.prefix,
            edge_iter: None,
        }
    }

    /// Returns None if there are no more elements to yield under this node, otherwise return
    /// Ok(value) if there is a value to yield, or Err(new_elem) if there is an underlying
    /// element to consider.
    fn advance(&mut self) -> Option<Result<&'a V, IterPath<'a, V>>> {
        if self.edge_iter.is_none() {
            self.edge_iter = Some(self.node.edges.iter());
            if let Some(ref value) = self.node.value {
                return Some(Ok(value));
            }
        }

        self.edge_iter.as_mut().unwrap().next()
            .map(IterPath::from_edge)
            .map(Err)
    }
}

pub struct Matches<'a, V: 'a> {
    result: Option<(String, Iter<'a, V>)>
}

impl<'a, V: 'a> Matches<'a, V> {
    fn found(prefix: String, node: &'a Node<V>) -> Matches<'a, V> {
        Matches {
            result: Some((prefix, node.iter())),
        }
    }

    fn none() -> Matches<'a, V> {
        Matches {
            result: None,
        }
    }
}

impl<'a, V: 'a> Iterator for Matches<'a, V> {
    type Item = (String, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.result.as_mut().and_then(|&mut (ref prefix, ref mut it)| {
            it.next().map(|(mut s, v)| {
                // prepend the common prefix
                s.insert_str(0, prefix);

                (s, v)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;

    #[test]
    fn it_can_be_constructed() {
        let t = Tree::<()>::new();
        assert!(t.is_empty());
    }

    #[test]
    fn it_maps_elements() {
        let mut t = Tree::new();
        t.insert("abc", 0);
        t.insert("others", 1);
        t.insert("other", 2);
        t.insert("othello", 3);

        assert_eq!(t.get("abc"), Some(&0));
        assert_eq!(t.get("others"), Some(&1));
        assert_eq!(t.get("other"), Some(&2));
        assert_eq!(t.get("othello"), Some(&3));
    }

    #[test]
    fn it_is_empty_after_being_cleared() {
        let mut t = Tree::new();
        t.insert("foo", ());
        t.insert("bar", ());
        t.insert("baz", ());

        // before clear
        assert!(!t.is_empty());
        assert!(t.get("foo").is_some());
        assert!(t.get("bar").is_some());
        assert!(t.get("baz").is_some());

        t.clear();

        // after clear
        assert!(t.is_empty());
        assert!(t.get("foo").is_none());
        assert!(t.get("bar").is_none());
        assert!(t.get("baz").is_none());
    }

    #[test]
    fn it_handles_adding_existing_parts() {
        let mut t = Tree::new();
        t.insert("abc", "long");
        t.insert("ab", "shorter");
        t.insert("a", "short");

        assert_eq!(t.get("abc").map(|s| s.to_string()), Some("long".to_string()));
        assert_eq!(t.get("ab").map(|s| s.to_string()), Some("shorter".to_string()));
        assert_eq!(t.get("a").map(|s| s.to_string()), Some("short".to_string()));
    }

    #[test]
    fn it_can_remove_keys() {
        let mut t = Tree::new();
        t.insert("abc", "long");
        t.insert("ab", "shorter");
        t.insert("a", "short");

        t.remove("ab");
        assert_eq!(t.get("ab"), None);

        t.remove("abc");
        assert_eq!(t.get("abc"), None);

        t.remove("a");
        assert_eq!(t.get("a"), None);

        assert!(t.is_empty());
    }

    #[test]
    fn it_can_iterate_on_items() {
        let items = vec![
            ("abc", 0),
            ("ac",  1),
            ("bc",  2),
            ("a",   3),
            ("ab",  4),
        ];

        let mut tree = Tree::new();
        for &(key, value) in items.iter() {
            tree.insert(key, value);
        }

        for &(key, value) in items.iter() {
            assert_eq!(tree.get(key).map(|v| *v), Some(value));
        }

        let mut got_items: Vec<_> = tree.iter().map(|(k, v)| (k, *v)).collect();
        got_items.sort();

        let mut items: Vec<_> = items.iter().map(|&(k, v)| (k.to_string(), v)).collect();
        items.sort();

        assert_eq!(got_items, items);
    }

    #[test]
    fn it_can_complete_a_prefix() {
        let items = vec![
            "apes",
            "apples",
            "apricots",
            "asteroids",
            "babies",
            "bananas",
            "glasses",
            "oranges",
        ];

        let t = {
            let mut t = Tree::new();
            for item in items.iter() {
                t.insert(item, ());
            }
            t
        };

        let matches: Vec<_> = t.find("ap").map(|(k, _)| k).collect();
        assert_eq!(matches, vec!["apes", "apples", "apricots"]);
    }

    #[test]
    fn it_has_sorted_iterators() {
        let items = vec![
            "c",
            "b",
            "a",
        ];

        let mut tree = Tree::new();
        for key in items.iter() {
            tree.insert(key, ());
        }

        let found: Vec<_> = tree.iter().map(|(k, _)| k).collect();
        assert_eq!(found, vec!["a", "b", "c"]);
    }
}
