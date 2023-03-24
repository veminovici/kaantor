use crate::ActorId;
use actix::prelude::*;

/// A message which returns the neighbours for a given actor.
#[derive(Message)]
#[rtype(result = "Vec<ActorId>")]
pub struct GetNeighbours(ActorId);

impl GetNeighbours {
    pub(crate) fn aid(&self) -> &ActorId {
        &self.0
    }
}

impl From<ActorId> for GetNeighbours {
    fn from(aid: ActorId) -> Self {
        Self(aid)
    }
}

/// A message to add an edge between two nodes
#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct AddBiEdge(ActorId, ActorId);

impl AddBiEdge {
    pub fn new(a: ActorId, b: ActorId) -> Self {
        Self(a, b)
    }

    pub fn a(&self) -> &ActorId {
        &self.0
    }

    pub fn b(&self) -> &ActorId {
        &self.1
    }
}
