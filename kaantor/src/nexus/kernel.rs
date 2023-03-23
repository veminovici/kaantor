use actix::prelude::*;
use log::{debug, info};

use crate::{ActorId, ProtocolPxy};

use super::message::{AddBiEdge, AddNode, GetNeighbours};

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

impl<P: Send + Unpin + 'static> Supervised for Kernel<P> {
    fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {
        debug!("{:?} | RESTART", self.aid());
    }
}

impl<P: Send + Unpin + 'static> SystemService for Kernel<P> {
    fn service_started(&mut self, ctx: &mut Context<Self>) {
        debug!("{:?} | STARTED | {:?} | {:?}", self.aid(), self.proxies.len(), ctx.state())
    }
}

impl<P: Send + Unpin + 'static> Handler<GetNeighbours> for Kernel<P> {
    type Result = <GetNeighbours as Message>::Result;

    fn handle(&mut self, msg: GetNeighbours, _ctx: &mut Self::Context) -> Self::Result {
        info!("RCVD | {:?} | GET_NEIGHBOURS | {:?}", self.aid(), msg.aid());
        vec![ActorId::new_kernel("test"), ActorId::new_node(1000)]
    }
}

impl<P: Send + Unpin + 'static> Handler<AddNode<P>> for Kernel<P> {
    type Result = <AddNode<P> as Message>::Result;

    fn handle(&mut self, msg: AddNode<P>, _ctx: &mut Self::Context) -> Self::Result {
        info!("RCVD | {:?} | ADD_NODE | {:?}", self.aid(), msg.aid());

        let pxy = msg.into_proxy();
        self.proxies.push(pxy);

        debug!("PXYS | {:?} | {:?}", self.aid(), self.proxies.len());
    }
}

impl<P: Send + Unpin + 'static> Handler<AddBiEdge> for Kernel<P> {
    type Result = <AddBiEdge as Message>::Result;

    fn handle(&mut self, msg: AddBiEdge, _ctx: &mut Self::Context) -> Self::Result {
        info!(
            "RCVD | {:?} | ADD_EDGE | {:?} - {:?}",
            self.aid(),
            msg.a(),
            msg.b()
        );
    }
}
