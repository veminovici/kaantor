use std::fmt::Debug;

use actix::prelude::*;
use kaantor::{nexus, ActorId, IntoActorId, Node, ProtocolMsg, SenderId, SessionId};
use log::debug;

struct MyActor(pub ActorId);

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl IntoActorId for MyActor {
    fn aid(&self) -> ActorId {
        self.0
    }
}

enum MyPayload {
    Ping,
}

impl Default for MyPayload {
    fn default() -> Self {
        Self::Ping
    }
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

    fn create(aid: ActorId) -> Node<MyActor> {
        let node = MyActor(aid);
        let addr = node.start();

        Node::new(aid, addr)
    }

    // initialize system
    let _code = System::new().block_on(async {
        // STEP 1: Create the nodes
        let aid1 = ActorId::from(1);
        let node1 = create(aid1);

        let aid2 = ActorId::from(2);
        let node2 = create(aid2);

        let aid3 = ActorId::from(3);
        let node3 = create(aid3);

        // STEP 2: Create the edges between the nodes
        let _ = nexus::add_edge::<MyPayload>(node1.aid(), node2.aid()).await;
        let _ = nexus::add_edge::<MyPayload>(node1.aid(), node3.aid()).await;

        // STEP 3: Add the proxies
        let _ = node1.register_proxy().await;
        let _ = node2.register_proxy().await;
        let _ = node3.register_proxy().await;

        // STEP 4: Start the protocol
        let sid = SenderId::from(ActorId::default());
        let kid = SessionId::from(10);
        let _ = node1.send(sid, kid, MyPayload::Ping).await;
    });
}
