use crate::Node;

pub struct Graph<K> {
    nodes: Vec<Node<K>>,
}

impl<K> Graph<K> {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add_node(&mut self, node: Node<K>) {
        self.nodes.push(node)
    }

    pub fn add_biedge(&mut self, a: K, b: K)
    where
        K: PartialEq + Copy,
    {
        self.add_edge(a, b);
        self.add_edge(b, a);
    }

    fn add_edge(&mut self, key: K, cid: K)
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

    pub fn neighbours(&self, key: K) -> impl Iterator<Item = &K>
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

impl<K> Default for Graph<K> {
    fn default() -> Self {
        Self::new()
    }
}
