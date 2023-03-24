use crate::ActorId;

use super::Node;

pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node)
    }

    pub fn add_biedge(&mut self, a: ActorId, b: ActorId) {
        self.add_edge(a, b);
        self.add_edge(b, a);
    }

    fn add_edge(&mut self, nid: ActorId, cid: ActorId) {
        self.nodes
            .iter_mut()
            .find(|n| n.aid() == &nid)
            .map(|n| n.add_edge_to(cid))
            .or({
                let n = Node::with_child(nid, cid);
                self.nodes.push(n);

                Some(())
            })
            .unwrap()
    }

    pub fn neighbours(&self, nid: ActorId) -> impl Iterator<Item = &ActorId> {
        self.nodes
            .iter()
            .find(|node| node.aid() == &nid)
            .map(|node| node.neighbours())
            .unwrap()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
