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
            .map(|n| n.add_child(cid))
            .unwrap()
    }

    pub fn neighbours(&self, nid: ActorId) -> impl Iterator<Item = &ActorId> {
        self.nodes
            .iter()
            .find(|n| n.aid() == &nid)
            .map(|n| n.children())
            .unwrap()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
