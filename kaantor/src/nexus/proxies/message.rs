use crate::{ActorId, ProtocolMsg, ProtocolPxy, SessionId};
use actix::prelude::*;
use std::fmt::Debug;

/// A message to add a node to the internal graph
#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct AddProxy<P: Send>(ActorId, Recipient<ProtocolMsg<P>>);

impl<P: Send> AddProxy<P> {
    pub fn new(aid: ActorId, recipient: Recipient<ProtocolMsg<P>>) -> Self {
        Self(aid, recipient)
    }

    pub(crate) fn aid(&self) -> &ActorId {
        &self.0
    }

    pub(crate) fn into_proxy(self) -> ProtocolPxy<P> {
        ProtocolPxy::new(self.0, self.1)
    }
}

pub(crate) enum SendTo {
    Actors(Vec<ActorId>),
}

impl Debug for SendTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Actors(xs) => write!(f, "SEND {xs:?}"),
        }
    }
}

/// A message to trigger a send the payload.
#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct SendPayload<P> {
    from: ActorId,
    to: SendTo,
    kid: SessionId,
    pld: P,
}

impl<P> SendPayload<P> {
    pub fn new(from: ActorId, to: SendTo, kid: SessionId, pld: P) -> Self {
        Self { from, to, kid, pld }
    }

    pub fn from(&self) -> ActorId {
        self.from
    }

    pub fn kid(&self) -> SessionId {
        self.kid
    }

    pub fn to(&self) -> &SendTo {
        &self.to
    }

    pub fn payload(&self) -> &P {
        &self.pld
    }
}
