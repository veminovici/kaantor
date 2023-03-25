use super::message::{AddProxy, SendPayload};
use crate::{
    nexus::proxies::message::SendTo, ActorId, IntoActorId, ProtocolMsg, ProtocolPxy, SenderId,
};
use actix::prelude::*;
// use futures::future::join_all;
use log::debug;
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

        debug!(
            "RCVD | {:?} >> {:?} | NODE+ | {:?}",
            ActorId::default(),
            self.aid(),
            aid
        );

        let pxy = msg.into_proxy();
        self.proxies.push(pxy);
    }
}

impl<P: Copy + Debug + Send + Unpin + 'static> Handler<SendPayload<P>> for ProxiesActor<P> {
    type Result = <SendPayload<P> as Message>::Result;

    fn handle(&mut self, msg: SendPayload<P>, _ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "{:?} || RCVD | {:?} >> {:?} | {:?}",
            self.aid,
            msg.from(),
            msg.to(),
            msg.payload()
        );

        let from = msg.from();

        let proxies: Vec<_> = match msg.to() {
            SendTo::Actors(xs) => self
                .proxies
                .iter()
                .filter(|pxy| {
                    let aid = *pxy.aid();
                    aid != from && xs.contains(&aid)
                })
                .collect(),
        };

        let _: Vec<_> = proxies
            .iter()
            .map(|pxy| {
                let sid = SenderId::from(from);
                // let to = pxy.aid();
                let kid = msg.kid();
                let pld = msg.payload();

                // debug!(
                //     "{:?} || SEND | {:?} >> {:?} | {:?} | {:?}",
                //     self.aid, sid, to, kid, pld,
                // );

                let msg = ProtocolMsg::new(sid, kid, *pld);
                let _ = pxy.try_send(msg);
            })
            .collect();

        // let futures = self.proxies.iter().map(|pxy| {
        //     let msg = ProtocolMsg::new(sid, kid, pld);
        //     pxy.send(msg)
        // });

        //let fut = join_all(futures);
        //let _ = fut.into_actor(self).boxed_local();
    }
}
