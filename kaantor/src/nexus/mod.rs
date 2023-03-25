mod graph;
mod proxies;

pub use graph::*;
pub use proxies::*;

use crate::{ActorId, SessionId};
use std::fmt::Debug;

pub async fn send_to_neighbours<P>(arg: Option<(ActorId, SessionId, P)>)
where
    P: Copy + Debug + Send + Unpin + 'static,
{
    if let Some((from, kid, pld)) = arg {
        let ns = get_neighbours(from).await.unwrap();
        let _ = send(from, ns.iter().copied(), kid, pld).await;
    }
}
