mod kernel;
pub(crate) mod message;

use self::kernel::Kernel;
use crate::{ActorId, ProtocolMsg};
use actix::{dev::ToEnvelope, prelude::*};
use anyhow::{anyhow, Result};

pub async fn get_neighbours<P>(aid: ActorId) -> Result<Vec<ActorId>>
where
    P: Default + Send + Unpin + 'static,
{
    let kernel = Kernel::<P>::from_registry();
    let msg = message::GetNeighbours::from(aid);
    let res = kernel.send(msg).await;
    res.map_err(|e| anyhow!(e))
}

pub async fn add_node<A, P>(aid: ActorId, addr: &Addr<A>) -> Result<()>
where
    A: Actor,
    A: Handler<ProtocolMsg<P>>,
    <A as actix::Actor>::Context: ToEnvelope<A, ProtocolMsg<P>>,
    P: Default + Send + Unpin + 'static,
{
    let recipient = addr.clone().recipient::<ProtocolMsg<P>>();
    let kernel = Kernel::<P>::from_registry();
    let msg = message::AddNode::new(aid, recipient);
    kernel.send(msg).await.map_err(|e| anyhow!(e))
}
