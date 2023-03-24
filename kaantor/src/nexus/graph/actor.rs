use crate::{graph::Graph, ActorId, IntoActorId};
use actix::prelude::*;
use log::info;

use super::message::{AddBiEdge, GetNeighbours};

pub struct GraphActor {
    aid: ActorId,
    graph: Graph,
}

impl Default for GraphActor {
    fn default() -> Self {
        Self {
            aid: ActorId::KrnlId("KRNL:GRPH"),
            graph: Default::default(),
        }
    }
}

impl IntoActorId for GraphActor {
    fn aid(&self) -> ActorId {
        self.aid
    }
}

impl Actor for GraphActor {
    type Context = Context<Self>;
}

impl Supervised for GraphActor {}

impl SystemService for GraphActor {}

impl Handler<GetNeighbours> for GraphActor {
    type Result = <GetNeighbours as Message>::Result;

    fn handle(&mut self, msg: GetNeighbours, _ctx: &mut Self::Context) -> Self::Result {
        let aid = msg.aid();
        info!(
            "RCVD | {:?} >> {:?} | BOURS | {:?}",
            ActorId::default(),
            self.aid(),
            msg.aid()
        );

        self.graph.neighbours(*aid).copied().collect()
    }
}

impl Handler<AddBiEdge> for GraphActor {
    type Result = <AddBiEdge as Message>::Result;

    fn handle(&mut self, msg: AddBiEdge, _ctx: &mut Self::Context) -> Self::Result {
        info!(
            "RCVD | {:?} >> {:?} | EDGE+ | {:?} <> {:?}",
            ActorId::default(),
            self.aid(),
            msg.a(),
            msg.b()
        );

        let a = *msg.a();
        let b = *msg.b();
        self.graph.add_biedge(a, b);
    }
}
