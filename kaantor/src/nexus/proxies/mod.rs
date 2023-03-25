use self::actor::ProxiesActor;
use crate::{ActorId, ProtocolMsg, SessionId};
use actix::{dev::ToEnvelope, prelude::*};
use anyhow::{anyhow, Result};
use std::fmt::Debug;

mod actor;
mod message;

pub(crate) async fn add_proxy<A, P>(aid: ActorId, addr: &Addr<A>) -> Result<()>
where
    A: Actor,
    A: Handler<ProtocolMsg<P>>,
    <A as actix::Actor>::Context: ToEnvelope<A, ProtocolMsg<P>>,
    P: Send + Unpin + 'static,
{
    let recipient = addr.clone().recipient::<ProtocolMsg<P>>();
    let kernel = ProxiesActor::<P>::from_registry();
    let msg = message::AddProxy::new(aid, recipient);
    kernel.send(msg).await.map_err(|e| anyhow!(e))
}

pub async fn send<P>(
    from: ActorId,
    actors: impl Iterator<Item = ActorId>,
    kid: SessionId,
    pld: P,
) -> Result<()>
where
    P: Copy + Debug + Send + Unpin + 'static,
{
    let actor = ProxiesActor::<P>::from_registry();
    let msg = message::SendPayload::new(from, message::SendTo::Actors(actors.collect()), kid, pld);

    actor.send(msg).await.map_err(|e| anyhow!(e))
}
