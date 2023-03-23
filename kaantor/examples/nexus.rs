use actix::prelude::*;
use kaantor::{nexus, ActorId, ProtocolMsg, SenderId, SessionId};
use log::debug;

struct MyActor(pub ActorId);

impl Actor for MyActor {
    type Context = Context<Self>;
}

enum MyPayload {
    Ping,
}

impl Default for MyPayload {
    fn default() -> Self {
        Self::Ping
    }
}

impl Message for MyPayload {
    type Result = ();
}

impl Handler<ProtocolMsg<MyPayload>> for MyActor {
    type Result = ResponseActFuture<Self, <MyPayload as Message>::Result>;

    fn handle(&mut self, _msg: ProtocolMsg<MyPayload>, _ctx: &mut Self::Context) -> Self::Result {
        let me = self.0;
        async move {
            println!("{:?} handles async", me);
            let ns = nexus::get_neighbours::<usize>(me).await.unwrap();
            println!("{:?} processed ns={:?}", me, ns);
            () // this is the <Ping as Message>::Result.
        }
        .into_actor(self)
        .boxed_local()
    }
}

fn main() {
    env_logger::init();
    debug!("Starting the example NEXUS_GET");

    // initialize system
    let _code = System::new().block_on(async {
        let aid = ActorId::from(30);
        let me = MyActor(aid);
        let addr = me.start();

        // add the node to the kernel
        let _ = nexus::add_node(aid, &addr).await;

        // start the protocol
        let sid = SenderId::from(aid);
        let kid = SessionId::from(100);
        let msg = ProtocolMsg::new(sid, kid, MyPayload::Ping);

        let _ = addr.send(msg).await;
    });
}
