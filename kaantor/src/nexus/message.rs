use crate::{ActorId, ProtocolMsg, ProtocolPxy};
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
    fn from(value: ActorId) -> Self {
        Self(value)
    }
}

/// A message to add a node to the internal graph
#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct AddNode<P: Send>(ActorId, Recipient<ProtocolMsg<P>>);

impl<P: Send> AddNode<P> {
    pub fn new(aid: ActorId, recipient: Recipient<ProtocolMsg<P>>) -> Self {
        Self(aid, recipient)
    }

    pub fn aid(&self) -> &ActorId {
        &self.0
    }

    pub(crate) fn into_proxy(self) -> ProtocolPxy<P> {
        ProtocolPxy::new(self.0, self.1)
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
