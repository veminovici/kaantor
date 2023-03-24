use actix::prelude::*;
use anyhow::Result;
use kaantor::{nexus, ActorId, IntoActorId, Node, ProtocolMsg, SenderId, SessionId};
use log::debug;
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

enum MyPayload {
    Ping,
}

impl Debug for MyPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ping => write!(f, "PING"),
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
            debug!(
                "RCVD | {:?} >> {:?} | {:?} | {:?}",
                msg.sid(),
                me,
                msg.kid(),
                msg.payload()
            );

            let ns = nexus::get_neighbours(me).await.unwrap();
            println!("RCVD | {:?} | PING | ns={:?}", me, ns);

            () // this is the <Ping as Message>::Result.
        }
        .into_actor(self)
        .boxed_local()
    }
}

fn main() {
    env_logger::init();
    debug!("Starting the example NEXUS_GET");

    async fn create(aid: ActorId) -> Node<MyActor> {
        let node = MyActor::from(aid);
        let addr = node.start();

        let node = Node::new(aid, addr);
        let _ = node.register_proxy().await;

        node
    }

    // initialize system
    let _code = System::new().block_on(async {
        // STEP 1: Create the nodes
        let aid1 = ActorId::from(1);
        let node1 = create(aid1).await;

        let aid2 = ActorId::from(2);
        let node2 = create(aid2).await;

        let aid3 = ActorId::from(3);
        let node3 = create(aid3).await;

        // STEP 2: Create the edges between the nodes
        let _ = nexus::add_edge::<MyPayload>(node1.aid(), node2.aid()).await;
        let _ = nexus::add_edge::<MyPayload>(node1.aid(), node3.aid()).await;

        // STEP 3: Start the protocol
        let sid = SenderId::from(ActorId::default());
        let kid = SessionId::from(10);
        let _ = node1.send(sid, kid, MyPayload::Ping).await;
    });
}
