use actix::prelude::*;
use kaantor::{nexus, ActorId, IntoActorId, ProtocolMsg, SessionId};
use kaantor_derive::{BuildNode, IntoActorId};
use log::info;
use std::fmt::Debug;

pub type Token = usize;

#[derive(BuildNode, IntoActorId)]
#[payload(FloodingPld)]
pub struct FloodingNode {
    aid: ActorId,
    tkn: Option<Token>,
}

impl Actor for FloodingNode {
    type Context = Context<Self>;
}

impl From<ActorId> for FloodingNode {
    fn from(aid: ActorId) -> Self {
        Self { aid, tkn: None }
    }
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
        let me = self.aid();
        let kid = *msg.kid();
        let sid = *msg.sid();
        let pld = *msg.payload();

        info!(
            "{:?} || RCVD | {:?} >> {:?} | {:?} | {:?} | {:?}",
            &me, &sid, &me, &kid, &pld, self.tkn
        );

        async fn fwd(arg: Option<(ActorId, SessionId, Token)>) {
            if let Some((from, kid, tkn)) = arg {
                let ns = nexus::get_neighbours(from).await.unwrap();
                let _ = nexus::send(from, ns.iter().copied(), kid, FloodingPld::Forward(tkn)).await;
            }
        }

        let fut = match pld {
            FloodingPld::Start(tkn) => {
                self.tkn = Some(tkn);
                fwd(Some((me, kid, tkn)))
            }
            FloodingPld::Forward(tkn) => match self.tkn {
                Some(_) => {
                    self.tkn = Some(tkn);
                    fwd(None)
                }
                None => {
                    self.tkn = Some(tkn);
                    fwd(Some((me, kid, tkn)))
                }
            },
        };

        fut.into_actor(self).boxed_local()
    }
}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn protocol() {
        env_logger::init();
        info!("Starting the example NEXUS_GET");

        // initialize system
        let _code = System::new().block_on(async {
            // STEP 1: Create the nodes
            let node1 = FloodingNode::build(1.into()).await;
            let node2 = FloodingNode::build(2.into()).await;
            let node3 = FloodingNode::build(3.into()).await;

            // STEP 2: Create the edges between the nodes
            let _ = nexus::add_edge::<FloodingPld>(node1.aid(), node2.aid()).await; // 1 - 2
            let _ = nexus::add_edge::<FloodingPld>(node1.aid(), node3.aid()).await; // 1 - 3

            // STEP 3: Start the protocol
            let _ = node1.send(10.into(), FloodingPld::Start(12)).await;
        });
    }
}
