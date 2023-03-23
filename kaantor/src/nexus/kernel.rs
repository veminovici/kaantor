use actix::prelude::*;
use log::info;

use crate::{ActorId, KActor, ProtocolPxy};

use super::message::{AddBiEdge, AddNode, GetNeighbours};

pub(crate) struct Kernel<P: Send> {
    aid: ActorId,
    proxies: Vec<ProtocolPxy<P>>,
}

impl<P: Send> Default for Kernel<P> {
    fn default() -> Self {
        Self {
            aid: ActorId::KrnlId("KRNL"),
            proxies: vec![],
        }
    }
}

impl<P: Send> KActor for Kernel<P> {
    fn aid(&self) -> &ActorId {
        &self.aid
    }
}

impl<P: Send + Unpin + 'static> Actor for Kernel<P> {
    type Context = Context<Self>;
}

impl<P: Send + Unpin + 'static> Supervised for Kernel<P> {}

impl<P: Send + Unpin + 'static> SystemService for Kernel<P> {}

impl<P: Send + Unpin + 'static> Handler<GetNeighbours> for Kernel<P> {
    type Result = <GetNeighbours as Message>::Result;

    fn handle(&mut self, msg: GetNeighbours, _ctx: &mut Self::Context) -> Self::Result {
        info!(
            "RCVD | {:?} >> {:?} | BOURS | {:?} | {:?}",
            ActorId::default(),
            self.aid(),
            msg.aid(),
            self.proxies.len()
        );
        vec![ActorId::new_kernel("test"), ActorId::new_node(1000)]
    }
}

impl<P: Send + Unpin + 'static> Handler<AddNode<P>> for Kernel<P> {
    type Result = <AddNode<P> as Message>::Result;

    fn handle(&mut self, msg: AddNode<P>, _ctx: &mut Self::Context) -> Self::Result {
        info!(
            "RCVD | {:?} >> {:?} | NODE+ | {:?}",
            ActorId::default(),
            self.aid(),
            msg.aid()
        );

        let pxy = msg.into_proxy();
        self.proxies.push(pxy);
    }
}

impl<P: Send + Unpin + 'static> Handler<AddBiEdge> for Kernel<P> {
    type Result = <AddBiEdge as Message>::Result;

    fn handle(&mut self, msg: AddBiEdge, _ctx: &mut Self::Context) -> Self::Result {
        info!(
            "RCVD | {:?} >> {:?} | EDGE+ | {:?} <> {:?}",
            ActorId::default(),
            self.aid(),
            msg.a(),
            msg.b()
        );
    }
}
