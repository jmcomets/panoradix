use std::mem;

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

    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.edges.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        if key.is_empty() {
            self.value.as_ref()
        } else {
            if let Some((i, PrefixCmp::Full(suffix))) = self.search_for_prefix(key) {
                self.edges[i].node.get(suffix)
            } else {
                None
            }
        }
    }

    pub fn insert(&mut self, key: &str, value: V) -> Option<V> {
        if key.is_empty() {
            let old_value = self.value.take();
            self.value = Some(value);
            old_value
        } else {
            if let Some((i, cmp)) = self.search_for_prefix(key) {
                let ref mut edge = self.edges[i];
                match cmp {
                    // Full prefix: insert in the child
                    PrefixCmp::Full(suffix) => {
                        return edge.node.insert(suffix, value);
                    },

                    // Partial prefix: split the key and replace the edge's node with a new one
                    // that holds both nodes to insert.
                    PrefixCmp::Partial(i) => {
                        edge.split_insert(i, key, value);
                    },
                };
            } else {
                // No match in edges: insert a new edge
                self.edges.push(Edge::new(key.to_string(), Some(value)));
            }

            None
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
        //self.node.edges.sort_by(|a, b| a.prefix.cmp(&b.prefix));
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
    use self::PrefixCmp::*;

    let nb = haystack.chars().zip(needle.chars())
        .take_while(|&(a, b)| a == b)
        .count();
    if nb == 0 {
        None
    } else if nb < haystack.len() {
        Some(Partial(nb))
    } else if nb < needle.len() {
        let (_, suffix) = needle.split_at(nb);
        Some(Full(suffix))
    } else {
        Some(Full(""))
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;

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
}
