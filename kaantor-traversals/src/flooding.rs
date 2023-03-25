use actix::prelude::*;
use kaantor::{nexus, ActorId, ProtocolMsg, SessionId};
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
        let t = format!("{:?}", &self.tkn);
        let (me, _sid, kid, pld) = msg.decompose_log(self, "RCVD", t.as_str());

        async fn continuation(arg: Option<(ActorId, SessionId, Token)>) {
            if let Some((from, kid, tkn)) = arg {
                let ns = nexus::get_neighbours(from).await.unwrap();
                let _ = nexus::send(from, ns.iter().copied(), kid, FloodingPld::Forward(tkn)).await;
            }
        }

        let fut = match pld {
            FloodingPld::Start(tkn) => {
                self.tkn = Some(tkn);
                continuation(Some((me, kid, tkn)))
            }
            FloodingPld::Forward(tkn) => match self.tkn {
                Some(_) => continuation(None),
                None => {
                    self.tkn = Some(tkn);
                    continuation(Some((me, kid, tkn)))
                }
            },
        };

        fut.into_actor(self).boxed_local()
    }
}
