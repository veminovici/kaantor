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

impl<K> From<K> for Tree<K> {
    fn from(root: K) -> Self {
        Self::new(root)
    }
}
