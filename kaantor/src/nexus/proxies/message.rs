use crate::{ActorId, ProtocolMsg, ProtocolPxy};
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
    Actor(ActorId),
    All,
    AllExcept(Vec<ActorId>),
}

impl Debug for SendTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Actor(aid) => write!(f, "{aid:?}"),
            Self::All => write!(f, "ALL"),
            Self::AllExcept(_) => write!(f, "A--"),
        }
    }
}

/// A message to trigger a send the payload.
#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct SendPayload<P>(ActorId, SendTo, P);

impl<P> SendPayload<P> {
    pub fn new(from: ActorId, to: SendTo, pld: P) -> Self {
        Self(from, to, pld)
    }

    pub fn to(&self) -> &SendTo {
        &self.1
    }

    pub fn payload(&self) -> &P {
        &self.2
    }
}
