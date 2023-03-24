use super::message::{AddProxy, SendPayload};
use crate::{ActorId, IntoActorId, ProtocolPxy};
use actix::prelude::*;
use log::info;
use std::fmt::Debug;

pub(crate) struct ProxiesActor<P: Send> {
    aid: ActorId,
    proxies: Vec<ProtocolPxy<P>>,
}

impl<P: Send> Default for ProxiesActor<P> {
    fn default() -> Self {
        Self {
            aid: ActorId::KrnlId("KRNL:PRXY"),
            proxies: vec![],
        }
    }
}

impl<P: Send> IntoActorId for ProxiesActor<P> {
    fn aid(&self) -> ActorId {
        self.aid
    }
}

impl<P: Send + Unpin + 'static> Actor for ProxiesActor<P> {
    type Context = Context<Self>;
}

impl<P: Send + Unpin + 'static> Supervised for ProxiesActor<P> {}

impl<P: Send + Unpin + 'static> SystemService for ProxiesActor<P> {}

impl<P: Send + Unpin + 'static> Handler<AddProxy<P>> for ProxiesActor<P> {
    type Result = <AddProxy<P> as Message>::Result;

    fn handle(&mut self, msg: AddProxy<P>, _ctx: &mut Self::Context) -> Self::Result {
        let aid = *msg.aid();

        info!(
            "RCVD | {:?} >> {:?} | NODE+ | {:?}",
            ActorId::default(),
            self.aid(),
            aid
        );

        let pxy = msg.into_proxy();
        self.proxies.push(pxy);
    }
}

impl<P: Debug + Send + Unpin + 'static> Handler<SendPayload<P>> for ProxiesActor<P> {
    type Result = <SendPayload<P> as Message>::Result;

    fn handle(&mut self, msg: SendPayload<P>, _ctx: &mut Self::Context) -> Self::Result {
        info!(
            "RCVD | {:?} >> {:?} | SEND | {:?}",
            self.aid(),
            msg.to(),
            msg.payload()
        )
    }
}
