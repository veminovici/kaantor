use actix::prelude::*;
use kaantor::{nexus, ActorId, IntoActorId, Node, ProtocolMsg};
use log::{debug, info};
use std::fmt::Debug;

struct MyActor(pub ActorId);

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

        async move {
            let kid = msg.kid();

            info!(
                "{:?} || RCVD | {:?} >> {:?} | {:?} | {:?}",
                me,
                msg.sid(),
                me,
                msg.kid(),
                msg.payload()
            );

            match msg.payload() {
                MyPayload::Start(tkn) => {
                    //let ns = nexus::get_neighbours(me).await.unwrap();
                    let _ = nexus::send_to_all_neighbours(me, *kid, MyPayload::Ping(*tkn)).await;
                }
                MyPayload::Ping(tkn) => {
                    // debug!("DUMP | {:?} | PING | {:04?}", me, tkn);

                    let to = msg.sid().aid();
                    let _ = nexus::send_to_actor(me, to, *kid, MyPayload::Pong(*tkn + 1)).await;
                }
                MyPayload::Pong(tkn) => {
                    debug!("{:?} || DONE | {:04?}", me, tkn);
                }
            }

            // let ns = nexus::get_neighbours(me).await.unwrap();
            // println!("RCVD | {:?} | PING | ns={:?}", me, ns);

            () // this is the <Ping as Message>::Result.
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
    debug!("Starting the example NEXUS_GET");

    async fn create(aid: ActorId) -> Node<MyActor> {
        let node = MyActor::from(aid);
        let addr = node.start();

        let node = Node::new(aid, addr);
        let _ = node.register_proxy::<MyPayload>().await;
        let _ = node.register_proxy::<MyPayload2>().await;

        node
    }

    // initialize system
    let _code = System::new().block_on(async {
        // STEP 1: Create the nodes
        let node1 = create(1.into()).await;
        let node2 = create(2.into()).await;
        let node3 = create(3.into()).await;

        // STEP 2: Create the edges between the nodes
        let _ = nexus::add_edge::<MyPayload>(node1.aid(), node2.aid()).await; // 1 - 2
        let _ = nexus::add_edge::<MyPayload>(node1.aid(), node3.aid()).await; // 1 - 3

        // STEP 3: Start the protocol
        let _ = node1.send(10.into(), MyPayload::Start(12)).await;
    });
}
