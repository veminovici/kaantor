use kaantor_derive::*;

#[derive(::actix::Message)]
#[rtype(result = "()")]
struct MyPayloadA;

#[derive(::actix::Message)]
#[rtype(result = "()")]
struct MyPayloadB;

#[derive(BuildActor)]
#[payload(MyPayloadA, MyPayloadB)]
struct MyActor(kaantor::ActorId);

impl ::actix::Actor for MyActor {
    type Context = actix::Context<Self>;
}

impl From<kaantor::ActorId> for MyActor {
    fn from(value: kaantor::ActorId) -> Self {
        Self(value)
    }
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
