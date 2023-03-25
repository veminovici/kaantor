use kaantor_derive::{BuildNode, FromActorId, IntoActorId};

#[derive(::actix::Message)]
#[rtype(result = "()")]
struct MyPayloadA;

#[derive(::actix::Message)]
#[rtype(result = "()")]
struct MyPayloadB;

#[derive(BuildNode, Default, FromActorId, IntoActorId)]
#[payload(MyPayloadA, MyPayloadB)]
struct MyActor {
    aid: kaantor::ActorId,
}

impl ::actix::Actor for MyActor {
    type Context = actix::Context<Self>;
}

impl actix::Handler<::kaantor::ProtocolMsg<MyPayloadA>> for MyActor {
    type Result = ();

    fn handle(
        &mut self,
        _msg: ::kaantor::ProtocolMsg<MyPayloadA>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        todo!()
    }
}

impl actix::Handler<::kaantor::ProtocolMsg<MyPayloadB>> for MyActor {
    type Result = ();

    fn handle(
        &mut self,
        _msg: ::kaantor::ProtocolMsg<MyPayloadB>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        todo!()
    }
}

fn main() {
    println!("Testing works!");
}
