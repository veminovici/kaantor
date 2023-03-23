use std::fmt::Debug;

/// The unique identifier for an actor.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActorId {
    KrnlId(&'static str),
    NodeId(usize),
}

impl Debug for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KrnlId(name) => write!(f, "{name}"),
            Self::NodeId(aid) => write!(f, "A{aid:03}"),
        }
    }
}

impl Default for ActorId {
    fn default() -> Self {
        ActorId::KrnlId("app")
    }
}

impl ActorId {
    pub fn new_node(value: usize) -> Self {
        Self::NodeId(value)
    }

    pub fn new_kernel(name: &'static str) -> Self {
        Self::KrnlId(name)
    }
}

impl From<usize> for ActorId {
    fn from(value: usize) -> Self {
        ActorId::new_node(value)
    }
}
