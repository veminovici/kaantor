use ptree::{print_tree, TreeBuilder};
use std::fmt::Debug;

use crate::Node;

pub struct Tree<K> {
    root: K,
    nodes: Vec<Node<K>>,
}

impl<K> Tree<K> {
    pub fn new(root: K) -> Self {
        Self {
            root,
            nodes: vec![],
        }
    }

    pub fn add_node(&mut self, node: Node<K>) {
        self.nodes.push(node)
    }

    fn add_child(&mut self, key: K, cid: K)
    where
        K: PartialEq + Copy,
    {
        self.nodes
            .iter_mut()
            .find(|n| n.key() == &key)
            .map(|n| n.add_edge_to(cid))
            .or({
                let n = Node::with_child(key, cid);
                self.nodes.push(n);

                Some(())
            })
            .unwrap()
    }

    pub fn children(&self, key: K) -> impl Iterator<Item = &K>
    where
        K: PartialEq,
    {
        self.nodes
            .iter()
            .find(|node| node.key() == &key)
            .map(|node| node.neighbours())
            .unwrap()
    }
}

#[cfg(feature = "pretty")]
impl<K: Debug + PartialEq> Tree<K> {
    fn pretty_print_node<'a>(&self, tb: &'a mut TreeBuilder, key: &K) -> &'a mut TreeBuilder {
        self.nodes
            .iter()
            .find(|node| node.key() == key)
            .map(|node| {
                if node.is_leaf() {
                    tb.add_empty_child(format!("{:?}", key))
                } else {
                    let tb = tb.begin_child(format!("{:?}", key));
                    let tb = node
                        .neighbours()
                        .fold(tb, |tb, cid| self.pretty_print_node(tb, cid));
                    tb.end_child()
                }
            })
            .unwrap()
    }

    pub fn pretty_print(&self, title: &str) {
        let mut tb = TreeBuilder::new(title.to_owned());
        let tb = self.pretty_print_node(&mut tb, &self.root).build();
        let _ = print_tree(&tb).unwrap();
    }
}

impl<K> From<K> for Tree<K> {
    fn from(root: K) -> Self {
        Self::new(root)
    }
}
