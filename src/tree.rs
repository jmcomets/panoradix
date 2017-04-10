use path::Path;

pub struct Tree<P: Path, V> {
    root: Node<P, V>,
}

impl<P: Path, V> Tree<P, V> {
    pub fn new() -> Tree<P, V> {
        Tree {
            root: Node::empty(),
        }
    }

    pub fn from_items<Items, T, U>(items: Items) -> Tree<P, V>
        where Items: IntoIterator<Item=(T, U)>,
              T: Into<P>,
              U: Into<V>,
    {
        let mut tree = Tree::new();
        for (t, u) in items {
            tree.insert(t, u);
        }
        tree
    }

    pub fn insert<T, U>(&mut self, path: T, value: U)
        where T: Into<P>,
              U: Into<V>,
    {
        let path = path.into();
        let value = value.into();
        self.root.insert(&path, value);
    }

    pub fn get(&self, path: &P) -> Option<&V> {
        self.root.get(path)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }
}

struct Node<P, V> {
    path: P,
    value: Option<V>,
    children: Vec<Box<Node<P, V>>>,
}

impl<P: Path, V> Node<P, V> {
    fn empty() -> Node<P, V> {
        Node {
            path: P::default(),
            value: None,
            children: Vec::new(),
        }
    }

    fn boxed(path: P, value: V) -> Box<Node<P, V>> {
        Box::new(Node {
            path: path,
            value: Some(value),
            children: Vec::new(),
        })
    }

    fn is_empty(&self) -> bool {
        self.path.is_empty() && self.children.iter().all(|n| n.is_empty())
    }

    fn get(&self, path: &P) -> Option<&V> {
        match self.search(path) {
            NodeSearch::InChild(suffix, i) => self.children[i].get(&suffix),
            NodeSearch::Found(i)           => self.children[i].value.as_ref(),
            _                              => None,
        }
    }

    fn insert(&mut self, path: &P, value: V) {
        match self.search(path) {
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

    fn search(&self, path: &P) -> NodeSearch<P> {
        let suffix = self.path.suffix_with(path);
        if suffix.is_empty() {
            return NodeSearch::NotFound
        }

        // Note: this needs to be a &&P because we don't want to move the path (P) out of
        // context.
        let search = self.children.binary_search_by_key(&(&suffix), |n| &n.path);
        match search {
            Ok(i) => NodeSearch::Found(i),
            Err(i) => {
                let mut child_index = None;

                // left child
                if i > 0 {
                    let ref child = self.children[i - 1];
                    let child_has_common_prefix = child.path.has_common_prefix(&suffix);
                    if child_has_common_prefix {
                        child_index = Some(i - 1);
                    }
                }

                // right child
                if i + 1 < self.children.len() {
                    let ref child = self.children[i + 1];
                    let child_has_common_prefix = child.path.has_common_prefix(&suffix);
                    if child_has_common_prefix {
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

enum NodeSearch<P: Path> {
    Found(usize),
    InChild(P, usize),
    CanAddToChildren(P, usize),
    NotFound,
}
