use actix::prelude::*;
use log::debug;

use crate::{ActorId, ProtocolPxy};

use super::message::{AddNode, GetNeighbours};

pub(crate) struct Kernel<P: Send> {
    proxies: Vec<ProtocolPxy<P>>,
}

impl<P: Send> Default for Kernel<P> {
    fn default() -> Self {
        Self { proxies: vec![] }
    }
}

impl<P: Send> Kernel<P> {
    pub fn aid(&self) -> ActorId {
        ActorId::KrnlId("KRNL")
    }
}

impl<P: Send + Unpin + 'static> Actor for Kernel<P> {
    type Context = Context<Self>;
}

impl<P: Send + Unpin + 'static> Supervised for Kernel<P> {}

impl<P: Send + Unpin + 'static> SystemService for Kernel<P> {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        debug!("{:?} started", self.aid())
    }
}

impl<P: Send + Unpin + 'static> Handler<GetNeighbours> for Kernel<P> {
    type Result = <GetNeighbours as Message>::Result;

    fn handle(&mut self, msg: GetNeighbours, _ctx: &mut Self::Context) -> Self::Result {
        debug!("{:?} received GET_NEIGHBOURS {:?}", self.aid(), msg.aid());
        vec![ActorId::new_kernel("test"), ActorId::new_node(1000)]
    }
}

impl<P: Send + Unpin + 'static> Handler<AddNode<P>> for Kernel<P> {
    type Result = <AddNode<P> as Message>::Result;

    fn handle(&mut self, msg: AddNode<P>, _ctx: &mut Self::Context) -> Self::Result {
        debug!("{:?} received ADD_NODE {:?}", self.aid(), msg.aid());

        let pxy = msg.into_proxy();
        self.proxies.push(pxy);
    }
}
