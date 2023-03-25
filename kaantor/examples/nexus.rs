use actix::prelude::*;
use kaantor::{nexus, ActorId, IntoActorId, ProtocolMsg};
use kaantor_derive::BuildNode;
use log::{debug, info};
use std::fmt::Debug;

#[derive(BuildNode)]
#[payload(MyPayload, MyPayload2)]
struct MyActor(ActorId);

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl From<ActorId> for MyActor {
    fn from(aid: ActorId) -> Self {
        Self(aid)
    }
}

impl IntoActorId for MyActor {
    fn aid(&self) -> ActorId {
        self.0
    }
}

#[derive(Clone, Copy)]
enum MyPayload {
    Start(usize),
    Ping(usize),
    Pong(usize),
}

impl Debug for MyPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start(tkn) => write!(f, "START {:04?}", tkn),
            Self::Ping(tkn) => write!(f, "PING {:04?}", tkn),
            Self::Pong(tkn) => write!(f, "PONG {:04?}", tkn),
        }
    }
}

impl Message for MyPayload {
    type Result = ();
}

impl Handler<ProtocolMsg<MyPayload>> for MyActor {
    type Result = ResponseActFuture<Self, <MyPayload as Message>::Result>;

    fn handle(&mut self, msg: ProtocolMsg<MyPayload>, _ctx: &mut Self::Context) -> Self::Result {
        let me = self.aid();
        let kid = *msg.kid();
        let sid = *msg.sid();
        let pld = *msg.payload();

        info!(
            "{:?} || RCVD | {:?} >> {:?} | {:?} | {:?}",
            &me, &sid, &me, &kid, &pld
        );

        async move {
            match pld {
                MyPayload::Start(tkn) => {
                    let _ = nexus::send_to_all_neighbours(me, kid, MyPayload::Ping(tkn)).await;
                }
                MyPayload::Ping(tkn) => {
                    let to = sid.aid();
                    let _ = nexus::send_to_actor(me, to, kid, MyPayload::Pong(tkn + 1)).await;
                }
                MyPayload::Pong(tkn) => {
                    debug!("{:?} || DONE | {:04?}", me, tkn);
                }
            }
        }
        .into_actor(self)
        .boxed_local()
    }
}

struct MyPayload2;

impl Debug for MyPayload2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyPayload2").finish()
    }
}

impl Message for MyPayload2 {
    type Result = ();
}

impl Handler<ProtocolMsg<MyPayload2>> for MyActor {
    type Result = ResponseActFuture<Self, <MyPayload2 as Message>::Result>;

    fn handle(&mut self, _msg: ProtocolMsg<MyPayload2>, _ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

fn main() {
    env_logger::init();
    info!("Starting the example NEXUS_GET");

    // async fn create(aid: ActorId) -> Node<MyActor> {
    //     let node = MyActor::from(aid);
    //     let addr = node.start();
    //     let node = Node::new(aid, addr);
    //     let _ = node.register_proxy::<MyPayload>().await;
    //     let _ = node.register_proxy::<MyPayload2>().await;
    //     node
    // }

    // initialize system
    let _code = System::new().block_on(async {
        // STEP 1: Create the nodes
        let node1 = MyActor::build(1.into()).await;
        let node2 = MyActor::build(2.into()).await;
        let node3 = MyActor::build(3.into()).await;

        // STEP 2: Create the edges between the nodes
        let _ = nexus::add_edge::<MyPayload>(node1.aid(), node2.aid()).await; // 1 - 2
        let _ = nexus::add_edge::<MyPayload>(node1.aid(), node3.aid()).await; // 1 - 3

        // STEP 3: Start the protocol
        let _ = node1.send(10.into(), MyPayload::Start(12)).await;
    });
}
