use crate::{nexus, ActorId, IntoActorId, ProtocolMsg, SenderId, SessionId};

use actix::{dev::ToEnvelope, prelude::*};
use anyhow::{anyhow, Result};

pub struct Node<A: Actor> {
    aid: ActorId,
    addr: Addr<A>,
}

impl<A: Actor> Node<A> {
    pub fn new(aid: ActorId, addr: Addr<A>) -> Self {
        Self { aid, addr }
    }
}

impl<A: Actor> Node<A> {
    pub async fn register_proxy<P>(&self) -> Result<()>
    where
        A: Handler<ProtocolMsg<P>>,
        <A as actix::Actor>::Context: ToEnvelope<A, ProtocolMsg<P>>,
        P: Send + Unpin + 'static,
    {
        nexus::add_proxy(self.aid, &self.addr).await
    }

    pub async fn send<P>(&self, sid: SenderId, kid: SessionId, pld: P) -> Result<(), MailboxError>
    where
        A: Handler<ProtocolMsg<P>>,
        <A as actix::Actor>::Context: ToEnvelope<A, ProtocolMsg<P>>,
        P: Send + Unpin + 'static,
    {
        let msg = ProtocolMsg::new(sid, kid, pld);
        self.addr.send(msg).await
    }
}

impl<A: Actor> IntoActorId for Node<A> {
    fn aid(&self) -> ActorId {
        self.aid
    }
}
