mod kernel;
pub(crate) mod message;

use self::kernel::Kernel;
use crate::{ActorId, ProtocolMsg};
use actix::{dev::ToEnvelope, prelude::*};
use anyhow::{anyhow, Result};
use std::fmt::Debug;

pub async fn get_neighbours<P>(aid: ActorId) -> Result<Vec<ActorId>>
where
    P: Send + Unpin + 'static,
{
    let kernel = Kernel::<P>::from_registry();
    let msg = message::GetNeighbours::from(aid);
    let res = kernel.send(msg).await;
    res.map_err(|e| anyhow!(e))
}

pub async fn send_to_actor<P>(from: ActorId, to: ActorId, pld: P) -> Result<()>
where
    P: Debug + Send + Unpin + 'static,
{
    let kernel = Kernel::<P>::from_registry();
    let msg = message::SendPayload::new(from, message::SendTo::Actor(to), pld);
    let res = kernel.send(msg).await;
    res.map_err(|e| anyhow!(e))
}

pub async fn send_to_all<P>(from: ActorId, pld: P) -> Result<()>
where
    P: Debug + Send + Unpin + 'static,
{
    let kernel = Kernel::<P>::from_registry();
    let msg = message::SendPayload::new(from, message::SendTo::All, pld);
    let res = kernel.send(msg).await;
    res.map_err(|e| anyhow!(e))
}

pub async fn send_to_all_except<P>(
    from: ActorId,
    except: impl Iterator<Item = ActorId>,
    pld: P,
) -> Result<()>
where
    P: Debug + Send + Unpin + 'static,
{
    let kernel = Kernel::<P>::from_registry();
    let msg = message::SendPayload::new(from, message::SendTo::AllExcept(except.collect()), pld);
    let res = kernel.send(msg).await;
    res.map_err(|e| anyhow!(e))
}

pub async fn add_node<A, P>(aid: ActorId, addr: &Addr<A>) -> Result<()>
where
    A: Actor,
    A: Handler<ProtocolMsg<P>>,
    <A as actix::Actor>::Context: ToEnvelope<A, ProtocolMsg<P>>,
    P: Send + Unpin + 'static,
{
    let recipient = addr.clone().recipient::<ProtocolMsg<P>>();
    let kernel = Kernel::<P>::from_registry();
    let msg = message::AddNode::new(aid, recipient);
    kernel.send(msg).await.map_err(|e| anyhow!(e))
}

pub async fn add_edge<P>(a: ActorId, b: ActorId) -> Result<()>
where
    P: Send + Unpin + 'static,
{
    let kernel = Kernel::<P>::from_registry();
    let msg = message::AddBiEdge::new(a, b);
    kernel.send(msg).await.map_err(|e| anyhow!(e))
}
