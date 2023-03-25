use actix::prelude::*;
use kaantor::{nexus, ActorId, ProtocolMsg};
use kaantor_derive::{BuildNode, FromActorId, IntoActorId};
use std::fmt::Debug;

pub type Token = usize;

#[derive(BuildNode, IntoActorId, Default, FromActorId)]
#[payload(FloodingPld)]
pub struct FloodingNode {
    aid: ActorId,
    tkn: Option<Token>,
}

impl Actor for FloodingNode {
    type Context = Context<Self>;
}

#[derive(Clone, Copy)]
pub enum FloodingPld {
    Start(usize),
    Forward(usize),
}

impl Debug for FloodingPld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start(tkn) => write!(f, "START {tkn:04?}"),
            Self::Forward(tkn) => write!(f, "FORWARD {tkn:04?}"),
        }
    }
}

impl Handler<ProtocolMsg<FloodingPld>> for FloodingNode {
    type Result = ResponseActFuture<Self, <ProtocolMsg<FloodingPld> as Message>::Result>;

    fn handle(&mut self, msg: ProtocolMsg<FloodingPld>, _ctx: &mut Self::Context) -> Self::Result {
        let tkn_debug = format!("{:?}", &self.tkn);
        let (me, _sid, kid, pld) = msg.decompose_rcvd(self, tkn_debug.as_str());

        let fut = match pld {
            FloodingPld::Start(tkn) => {
                self.tkn = Some(tkn);
                let args = Some((me, kid, FloodingPld::Forward(tkn)));
                nexus::send_to_neighbours(args)
            }
            FloodingPld::Forward(tkn) => match self.tkn {
                Some(_) => nexus::send_to_neighbours(None),
                None => {
                    self.tkn = Some(tkn);
                    let args = Some((me, kid, FloodingPld::Forward(tkn)));
                    nexus::send_to_neighbours(args)
                }
            },
        };

        fut.into_actor(self).boxed_local()
    }
}
