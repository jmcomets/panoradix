use std::mem;
use std::slice;
use std::borrow::Cow;

use key::KeyComponent;

pub type Tree<K, V> = Node<K, V>;

trait PrefixExt<K> {
    fn add_prefix(&mut self, other: &[K]);

    fn add_suffix(&mut self, other: &[K]);
}

impl<K: Clone> PrefixExt<K> for Vec<K> {
    fn add_prefix(&mut self, other: &[K]) {
        let mut old = Vec::with_capacity(self.len() + other.len());
        mem::swap(self, &mut old);
        self.extend_from_slice(other);
        self.extend_from_slice(&old);
    }

    fn add_suffix(&mut self, other: &[K]) {
        self.extend_from_slice(other);
    }
}

pub struct Node<K: KeyComponent, V> {
    value: Option<V>,
    edges: Vec<Edge<K, V>>,
}

impl<K: KeyComponent, V> Node<K, V> {
    pub fn new() -> Node<K, V> {
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

    pub fn get(&self, key: &[K]) -> Option<&V> {
        if key.is_empty() {
            self.value.as_ref()
        } else if let Some((i, PrefixCmp::Full(suffix))) = self.search_for_prefix(key) {
            self.edges[i].node.get(&suffix)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: &[K], value: V) -> Option<V> {
        if key.is_empty() {
            let old_value = self.value.take();
            self.value = Some(value);
            old_value
        } else {
            if let Some((i, cmp)) = self.search_for_prefix(key) {
                match cmp {
                    // Full prefix: insert in the child
                    PrefixCmp::Full(suffix) => {
                        return self.edges[i].node.insert(&suffix, value);
                    },

                    // Partial prefix: split the key and replace the edge's node with a new one
                    // that holds both nodes to insert.
                    PrefixCmp::Partial(j) => {
                        self.edges[i].split_insert(j, key, value);
                    },
                };
            } else {
                // No match in edges: insert a new edge
                let new_edge = Edge::new(key.to_owned(), Some(value));

                // TODO: this should be revamped along with `search_for_prefix`
                let i = self.edges.binary_search_by(|e| e.prefix.as_slice().cmp(key)).unwrap_err();
                self.edges.insert(i, new_edge);
            }

            None
        }
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter::new(self)
    }

    pub fn remove(&mut self, key: &[K]) -> Option<V> {
        if key.is_empty() {
            self.value.take()
        } else if let Some((i, cmp)) = self.search_for_prefix(key) {
            match cmp {
                PrefixCmp::Full(suffix) => {
                    let ret = self.edges[i].node.remove(&suffix);

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

    pub fn find<'a>(&'a self, key: &[K]) -> Matches<'a, K, V> {
        self.find_subtree(key, Vec::new())
    }

    fn find_subtree<'a>(&'a self, key: &[K], mut prefix: Vec<K>) -> Matches<'a, K, V> {
        if key.is_empty() {
            Matches::found(prefix, self)
        } else if let Some((i, PrefixCmp::Full(suffix))) = self.search_for_prefix(key) {
            // concatenate the prefix used to get here with the current full prefix
            let new_prefix = {
                if suffix.is_empty() {
                    key
                } else {
                    let (p, _) = key.split_at(key.len() - suffix.len());
                    p
                }
            };

            prefix.add_suffix(new_prefix);

            self.edges[i].node.find_subtree(suffix.borrow(), prefix)
        } else {
            Matches::none()
        }
    }

    fn search_for_prefix<'a>(&self, key: &'a [K]) -> Option<(usize, PrefixCmp<'a, K>)> {
        self.edges.iter()
            .enumerate()
            .flat_map(|(i, e)| cmp_prefix(&e.prefix, key).map(|cmp| (i, cmp)))
            .next()
    }
}

struct Edge<K: KeyComponent, V> {
    prefix: Vec<K>,
    node: Box<Node<K, V>>,
}

impl<K: KeyComponent, V> Edge<K, V> {
    fn new(prefix: Vec<K>, value: Option<V>) -> Edge<K, V> {
        let mut node = Box::new(Node::new());
        node.value = value;

        Edge {
            prefix: prefix,
            node: node,
        }
    }

    fn split_insert(&mut self, i: usize, key: &[K], value: V) {
        let (prefix, (key_suffix, edge_suffix)) = {
            let (prefix, key_suffix) = key.split_at(i);
            let (_, edge_suffix) = self.prefix.split_at(i);

            (prefix.to_owned(), (key_suffix.to_owned(), edge_suffix.to_owned()))
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
}

enum PrefixCmp<'a, K: 'a + KeyComponent> {
    Full(Cow<'a, [K]>),
    Partial(usize),
}

fn cmp_prefix<'a, K: KeyComponent>(haystack: &[K], needle: &'a [K]) -> Option<PrefixCmp<'a, K>> {
    let nb = haystack.iter().zip(needle.iter())
        .take_while(|&(a, b)| a == b)
        .count();
    if nb == 0 {
        None
    } else if nb < haystack.len() {
        Some(PrefixCmp::Partial(nb))
    } else if nb < needle.len() {
        let (_, suffix) = needle.split_at(nb);
        Some(PrefixCmp::Full(Cow::Borrowed(suffix)))
    } else {
        Some(PrefixCmp::Full(Cow::default()))
    }
}

pub struct Iter<'a, K: 'a + KeyComponent, V: 'a> {
    path: Vec<IterPath<'a, K, V>>,
    prefix: Vec<K>,
}

impl<'a, K: KeyComponent, V: 'a> Iter<'a, K, V> {
    fn new(node: &'a Node<K, V>) -> Iter<'a, K, V> {
        Iter {
            path: vec![IterPath::from_node(node)],
            prefix: Vec::new(),
        }
    }
}

impl<'a, K: KeyComponent, V: 'a> Iterator for Iter<'a, K, V> {
    type Item = (Vec<K>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while !self.path.is_empty() {
            if let Some(adv) = self.path.last_mut().unwrap().advance() {
                match adv {
                    Ok(value) => {
                        return Some((self.prefix.clone(), value));
                    },
                    Err(elem) => {
                        self.prefix.add_suffix(&elem.prefix);
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

struct IterPath<'a, K: 'a + KeyComponent, V: 'a> {
    node: &'a Node<K, V>,
    edge_iter: Option<slice::Iter<'a, Edge<K, V>>>,
    prefix: Cow<'a, [K]>,
}

impl<'a, K: 'a + KeyComponent, V: 'a> IterPath<'a, K, V> {
    fn from_node(node: &'a Node<K, V>) -> IterPath<'a, K, V> {
        IterPath {
            node: node,
            prefix: Cow::default(),
            edge_iter: None,
        }
    }

    fn from_edge(edge: &'a Edge<K, V>) -> IterPath<'a, K, V> {
        IterPath {
            node: &edge.node,
            prefix: Cow::Borrowed(&edge.prefix),
            edge_iter: None,
        }
    }

    /// Returns None if there are no more elements to yield under this node, otherwise return
    /// Ok(value) if there is a value to yield, or Err(new_elem) if there is an underlying
    /// element to consider.
    fn advance(&mut self) -> Option<Result<&'a V, IterPath<'a, K, V>>> {
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

pub struct Matches<'a, K: 'a + KeyComponent, V: 'a> {
    result: Option<(Vec<K>, Iter<'a, K, V>)>
}

impl<'a, K: 'a + KeyComponent, V: 'a> Matches<'a, K, V> {
    fn found(prefix: Vec<K>, node: &'a Node<K, V>) -> Matches<'a, K, V> {
        Matches {
            result: Some((prefix, node.iter())),
        }
    }

    fn none() -> Matches<'a, K, V> {
        Matches {
            result: None,
        }
    }
}

impl<'a, K: 'a + KeyComponent, V: 'a> Iterator for Matches<'a, K, V> {
    type Item = (Vec<K>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.result.as_mut().and_then(|&mut (ref prefix, ref mut it)| {
            it.next().map(|(mut s, v)| {
                s.add_prefix(prefix);

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
        let t = Tree::<(), ()>::new();
        assert!(t.is_empty());
    }

    #[test]
    fn it_maps_elements() {
        let mut t = Tree::<u8, i32>::new();
        t.insert(b"abc", 0);
        t.insert(b"others", 1);
        t.insert(b"other", 2);
        t.insert(b"othello", 3);

        assert_eq!(t.get(b"abc"), Some(&0));
        assert_eq!(t.get(b"others"), Some(&1));
        assert_eq!(t.get(b"other"), Some(&2));
        assert_eq!(t.get(b"othello"), Some(&3));
    }

    #[test]
    fn it_is_empty_after_being_cleared() {
        let mut t = Tree::new();
        t.insert(b"foo", ());
        t.insert(b"bar", ());
        t.insert(b"baz", ());

        // before clear
        assert!(!t.is_empty());
        assert!(t.get(b"foo").is_some());
        assert!(t.get(b"bar").is_some());
        assert!(t.get(b"baz").is_some());

        t.clear();

        // after clear
        assert!(t.is_empty());
        assert!(t.get(b"foo").is_none());
        assert!(t.get(b"bar").is_none());
        assert!(t.get(b"baz").is_none());
    }

    #[test]
    fn it_handles_adding_existing_parts() {
        let mut t: Tree<u8, &'static str> = Tree::new();
        t.insert(b"abc", "long");
        t.insert(b"ab", "shorter");
        t.insert(b"a", "short");

        assert_eq!(t.get(b"abc").map(|s| s.to_string()), Some("long".to_string()));
        assert_eq!(t.get(b"ab").map(|s| s.to_string()), Some("shorter".to_string()));
        assert_eq!(t.get(b"a").map(|s| s.to_string()), Some("short".to_string()));
    }

    #[test]
    fn it_can_remove_keys() {
        let mut t = Tree::new();
        t.insert(b"abc", "long");
        t.insert(b"ab", "shorter");
        t.insert(b"a", "short");

        t.remove(b"ab");
        assert_eq!(t.get(b"ab"), None);

        t.remove(b"abc");
        assert_eq!(t.get(b"abc"), None);

        t.remove(b"a");
        assert_eq!(t.get(b"a"), None);

        assert!(t.is_empty());
    }

    #[test]
    fn it_can_iterate_on_items() {
        let items: Vec<(&'static [u8], i32)> = vec![
            (b"abc", 0),
            (b"ac",  1),
            (b"bc",  2),
            (b"a",   3),
            (b"ab",  4),
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

        let mut items: Vec<_> = items.iter().map(|&(k, v)| (k.to_owned(), v)).collect();
        items.sort();

        assert_eq!(got_items, items);
    }

    #[test]
    fn it_can_complete_a_prefix() {
        let items: Vec<&'static [u8]> = vec![
            b"apes",
            b"apples",
            b"apricots",
            b"asteroids",
            b"babies",
            b"bananas",
            b"glasses",
            b"oranges",
        ];

        let t = {
            let mut t = Tree::new();
            for item in items.iter() {
                t.insert(item, ());
            }
            t
        };

        let matches: Vec<_> = t.find(b"ap").map(|(k, _)| k).collect();
        let expected: Vec<&'static [u8]> = vec![b"apes", b"apples", b"apricots"];
        assert_eq!(matches, expected);
    }

    #[test]
    fn it_has_sorted_iterators() {
        let items: Vec<&'static [u8]> = vec![
            b"c",
            b"b",
            b"a",
        ];

        let mut tree = Tree::new();
        for key in items.iter() {
            tree.insert(key, ());
        }

        let found: Vec<_> = tree.iter().map(|(k, _)| k).collect();
        let expected: Vec<&'static [u8]> = vec![b"a", b"b", b"c"];
        assert_eq!(found, expected);
    }
}
