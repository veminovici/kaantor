use crate::ActorId;

pub struct Node {
    aid: ActorId,
    children: Vec<ActorId>,
}

impl Node {
    pub fn new(aid: ActorId) -> Self {
        Self {
            aid,
            children: vec![],
        }
    }

    pub fn with_child(aid: ActorId, cid: ActorId) -> Self {
        Self { aid, children: vec![cid] }
    }

    pub fn aid(&self) -> &ActorId {
        &self.aid
    }

    pub fn add_child(&mut self, aid: ActorId) {
        self.children.push(aid)
    }

    pub fn children(&self) -> impl Iterator<Item = &ActorId> {
        self.children.iter()
    }
}

impl From<ActorId> for Node {
    fn from(value: ActorId) -> Self {
        Node::new(value)
    }
}
