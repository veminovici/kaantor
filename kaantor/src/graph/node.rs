use crate::ActorId;

pub struct Node {
    aid: ActorId,
    ns: Vec<ActorId>,
}

impl Node {
    pub fn new(aid: ActorId) -> Self {
        Self {
            aid,
            ns: vec![],
        }
    }

    pub fn with_child(aid: ActorId, cid: ActorId) -> Self {
        Self { aid, ns: vec![cid] }
    }

    pub fn aid(&self) -> &ActorId {
        &self.aid
    }

    pub fn add_edge_to(&mut self, aid: ActorId) {
        self.ns.push(aid)
    }

    pub fn neighbours(&self) -> impl Iterator<Item = &ActorId> {
        self.ns.iter()
    }
}

impl From<ActorId> for Node {
    fn from(value: ActorId) -> Self {
        Node::new(value)
    }
}
