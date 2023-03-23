use std::fmt::Debug;

use actix::prelude::*;
use kaantor::{nexus, ActorId, KActor, ProtocolMsg, SenderId, SessionId};
use log::debug;

struct MyActor(pub ActorId);

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl KActor for MyActor {
    fn aid(&self) -> &ActorId {
        &self.0
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
        let me = *self.aid();
        async move {
            debug!(
                "RCVD | {:?} >> {:?} | {:?} | {:?}",
                msg.sid(),
                me,
                msg.kid(),
                msg.payload()
            );

            println!("{:?} handles async", me);
            let _ns = nexus::get_neighbours::<MyPayload>(me).await.unwrap();

            () // this is the <Ping as Message>::Result.
        }
        .into_actor(self)
        .boxed_local()
    }
}

fn main() {
    env_logger::init();
    debug!("Starting the example NEXUS_GET");

    fn create(aid: ActorId) -> Addr<MyActor> {
        let node = MyActor(aid);
        node.start()
    }

    // initialize system
    let _code = System::new().block_on(async {
        // Create the nodes
        let aid1 = ActorId::from(1);
        let node1 = create(aid1);
        let _ = nexus::add_node(aid1, &node1).await;

        let aid2 = ActorId::from(2);
        let node2 = create(aid2);
        let _ = nexus::add_node(aid2, &node2).await;

        // Create the edges between the nodes
        let _ = nexus::add_edge::<MyPayload>(aid1, aid2).await;

        // start the protocol
        let sid = SenderId::from(ActorId::default());
        let kid = SessionId::from(10);
        let msg = ProtocolMsg::new(sid, kid, MyPayload::Ping);

        let _ = node1.send(msg).await;
    });
}
