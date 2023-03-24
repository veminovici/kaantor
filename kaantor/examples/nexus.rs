use actix::prelude::*;
use kaantor::{nexus, ActorId, IntoActorId, Node, ProtocolMsg};
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
        let _ = node1
            .send(10.into(), MyPayload::Ping)
            .await;
    });
}
