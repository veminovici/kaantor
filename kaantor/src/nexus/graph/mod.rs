mod actor;
mod message;

use crate::ActorId;
use actix::SystemService;
use anyhow::{anyhow, Result};

/// Returns the list of neighbours for a given node.
pub async fn get_neighbours(aid: ActorId) -> Result<Vec<ActorId>> {
    let actor = actor::GraphActor::from_registry();
    let msg = message::GetNeighbours::from(aid);

    actor.send(msg).await.map_err(|e| anyhow!(e))
}

/// Adds a bi-directional edge between two nodes.
pub async fn add_edge<P>(a: ActorId, b: ActorId) -> Result<()>
where
    P: Send + Unpin + 'static,
{
    let actor = actor::GraphActor::from_registry();
    let msg = message::AddBiEdge::new(a, b);

    actor.send(msg).await.map_err(|e| anyhow!(e))
}
